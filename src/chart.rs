use crate::{
    aspect_ratio::{AspectRatioCalc, CalcUsing},
    bounds::Bounds,
    debug::DebugRect,
    inner::{InnerLayout, InnerOption},
    layout::{
        HorizontalLayout, HorizontalOption, UnconstrainedLayout, VerticalLayout, VerticalOption,
    },
    overlay::{OverlayLayout, UseOverlay},
    series::{Series, UseSeries},
    use_watched_node::{use_watched_node, UseWatchedNode},
    AspectRatio, Font, Padding,
};
use leptos::{html::Div, *};
use std::rc::Rc;

#[derive(Clone)]
pub struct Chart<X: 'static, Y: 'static> {
    debug: Option<MaybeSignal<bool>>,
    attr: Attr,

    top: Vec<Rc<dyn HorizontalOption<X, Y>>>,
    right: Vec<Rc<dyn VerticalOption<X, Y>>>,
    bottom: Vec<Rc<dyn HorizontalOption<X, Y>>>,
    left: Vec<Rc<dyn VerticalOption<X, Y>>>,
    inner: Vec<Rc<dyn InnerOption<X, Y>>>,
    overlay: Vec<Rc<dyn UseOverlay<X, Y>>>,
    series: UseSeries<X, Y>,
}

#[derive(Clone, Debug)]
pub struct Attr {
    pub font: MaybeSignal<Font>,
    pub padding: MaybeSignal<Padding>,
    pub debug: MaybeSignal<bool>,
}

impl<X, Y> Chart<X, Y> {
    pub fn new(font: impl Into<MaybeSignal<Font>>, series: UseSeries<X, Y>) -> Self {
        Self {
            debug: None,
            attr: Attr {
                font: font.into(),
                padding: MaybeSignal::default(),
                debug: MaybeSignal::default(),
            },

            top: vec![],
            right: vec![],
            bottom: vec![],
            left: vec![],
            inner: vec![],
            overlay: vec![],
            series,
        }
    }

    pub fn inherit_font(mut self, font: impl Into<MaybeSignal<Font>>) -> Self {
        self.attr.font = font.into();
        self
    }

    pub fn inherit_padding(mut self, padding: impl Into<MaybeSignal<Padding>>) -> Self {
        self.attr.padding = padding.into();
        self
    }

    pub fn set_debug(mut self, debug: impl Into<MaybeSignal<bool>>) -> Self {
        self.debug = Some(debug.into());
        self
    }
    pub fn inherit_debug(mut self, debug: impl Into<MaybeSignal<bool>>) -> Self {
        self.attr.debug = debug.into();
        self
    }

    pub fn top(mut self, opt: impl HorizontalLayout<X, Y>) -> Self {
        self.top.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn right(mut self, opt: impl VerticalLayout<X, Y>) -> Self {
        self.right.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn bottom(mut self, opt: impl HorizontalLayout<X, Y>) -> Self {
        self.bottom.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn left(mut self, opt: impl VerticalLayout<X, Y>) -> Self {
        self.left.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn inner(mut self, opt: impl InnerLayout<X, Y>) -> Self {
        self.inner.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn overlay(mut self, opt: impl OverlayLayout<X, Y>) -> Self {
        self.overlay.push(opt.apply_attr(&self.attr));
        self
    }
}

#[component]
pub fn Chart<X: Clone + 'static, Y: Clone + 'static>(
    chart: Chart<X, Y>,
    #[prop(into)] aspect_ratio: MaybeSignal<AspectRatio>,
) -> impl IntoView {
    let root = create_node_ref::<Div>();
    let watch = use_watched_node(root);

    // Note: unravel option here and put the test in the render function
    let width = create_memo(move |_| watch.bounds.get().unwrap_or_default().width());
    let height = create_memo(move |_| watch.bounds.get().unwrap_or_default().height());
    let calc = Signal::derive(move || match aspect_ratio.get().0 {
        CalcUsing::Env(calc) => watch
            .bounds
            .get()
            .map(move |_| calc.mk_signal(width, height)),
        CalcUsing::Known(calc) => Some(calc),
    });

    let render = move || {
        if let Some(calc) = calc.get() {
            view! {
                <RenderChart
                    chart=chart.clone()
                    watch=watch.clone()
                    aspect_ratio=calc />
            }
            .into_view()
        } else {
            view!(<p>"Loading..."</p>).into_view()
        }
    };

    view! {
        <div node_ref=root>
            {render}
        </div>
    }
}

#[component]
fn RenderChart<X: Clone + 'static, Y: Clone + 'static>(
    chart: Chart<X, Y>,
    watch: UseWatchedNode,
    aspect_ratio: AspectRatioCalc,
) -> impl IntoView {
    let Chart {
        debug,
        attr,

        top,
        right,
        bottom,
        left,
        inner,
        overlay,
        series,
    } = chart;
    let debug = debug.unwrap_or(attr.debug);

    // Add top / bottom options
    let layout = UnconstrainedLayout::horizontal_options(top, bottom);

    // Add left / right options
    let inner_height = aspect_ratio
        .clone()
        .height_signal(layout.top_height, layout.bottom_height);
    let layout = layout.vertical_options(left, right, &series, inner_height);

    // Compose chart
    let inner_width = aspect_ratio.width_signal(layout.left_width, layout.right_width);
    let outer_bounds = Signal::derive(move || {
        Bounds::new(
            layout.left_width.get() + inner_width.get() + layout.right_width.get(),
            layout.top_height.get() + inner_height.get() + layout.bottom_height.get(),
        )
    });
    let layout = layout.compose(outer_bounds, inner_width, &series);

    // Inner layout
    let inner = inner
        .into_iter()
        .map(|opt| {
            opt.into_use(&series, layout.projection)
                .render(layout.projection, &watch)
        })
        .collect_view();

    // Overlay
    let overlay = overlay
        .into_iter()
        .map(|opt| opt.render(series.clone(), layout.projection, &watch))
        .collect_view();

    view! {
        <svg
            width=move || format!("{}px", outer_bounds.get().width())
            height=move || format!("{}px", outer_bounds.get().height())
            viewBox=move || with!(|outer_bounds| format!("0 0 {} {}", outer_bounds.width(), outer_bounds.height()))
            style="display: block;">
            {inner}
            <DebugRect label="Chart" debug=debug bounds=move || vec![outer_bounds.get(), outer_bounds.get()] />
            {layout.view}
            <Series series=series projection=layout.projection />
        </svg>
        {overlay}
    }
}
