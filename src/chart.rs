use crate::{
    aspect_ratio::{AspectRatioCalc, CalcUsing},
    bounds::Bounds,
    debug::DebugRect,
    inner::{InnerLayout, InnerOption},
    layout::{HorizontalLayout, UnconstrainedLayout, VerticalLayout},
    overlay::{OverlayLayout, UseOverlay},
    series::{Series, UseSeries},
    state::{AttrState, State},
    use_watched_node::{use_watched_node, UseWatchedNode},
    AspectRatio, Font, Padding,
};
use leptos::{html::Div, *};
use std::rc::Rc;

#[derive(Clone)]
pub struct Chart<X: 'static, Y: 'static> {
    attr: AttrState,

    top: Vec<Rc<dyn HorizontalLayout<X, Y>>>,
    right: Vec<Rc<dyn VerticalLayout<X, Y>>>,
    bottom: Vec<Rc<dyn HorizontalLayout<X, Y>>>,
    left: Vec<Rc<dyn VerticalLayout<X, Y>>>,
    inner: Vec<Rc<dyn InnerOption<X, Y>>>,
    overlay: Vec<Rc<dyn UseOverlay<X, Y>>>,
    series: UseSeries<X, Y>,
}

impl<X, Y> Chart<X, Y> {
    pub fn new(
        debug: impl Into<Signal<bool>>,
        padding: impl Into<Signal<Padding>>,
        font: impl Into<Signal<Font>>,
        series: UseSeries<X, Y>,
    ) -> Self {
        Self {
            attr: AttrState {
                debug: debug.into(),
                font: font.into(),
                padding: padding.into(),
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

    pub fn top(mut self, opt: impl HorizontalLayout<X, Y> + 'static) -> Self {
        self.top.push(Rc::new(opt));
        self
    }

    pub fn right(mut self, opt: impl VerticalLayout<X, Y> + 'static) -> Self {
        self.right.push(Rc::new(opt));
        self
    }

    pub fn bottom(mut self, opt: impl HorizontalLayout<X, Y> + 'static) -> Self {
        self.bottom.push(Rc::new(opt));
        self
    }

    pub fn left(mut self, opt: impl VerticalLayout<X, Y> + 'static) -> Self {
        self.left.push(Rc::new(opt));
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
        <div node_ref=root style="width: fit-content; height: fit-content; overflow: visible;">
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
        attr,

        top,
        right,
        bottom,
        left,
        inner,
        overlay,
        series,
    } = chart;
    let debug = attr.debug;

    // Add top / bottom options
    let layout = UnconstrainedLayout::horizontal_options(top, bottom, &attr);

    // Add left / right options
    let inner_height = aspect_ratio
        .clone()
        .inner_height_signal(layout.top_height, layout.bottom_height);
    let layout = layout.vertical_options(left, right, &attr, &series, inner_height);

    // Compose chart
    let inner_width = aspect_ratio.inner_width_signal(layout.left_width, layout.right_width);
    let outer_bounds = Signal::derive(move || {
        Bounds::new(
            layout.left_width.get() + inner_width.get() + layout.right_width.get(),
            layout.top_height.get() + inner_height.get() + layout.bottom_height.get(),
        )
    });
    let layout = layout.compose(outer_bounds, inner_width, &attr, &series);
    let state = State::new(attr, layout.projection, &watch);

    // Edge layout
    let edges = layout.render_edges(state.clone());

    // Inner layout
    let inner = inner
        .into_iter()
        .map(|opt| opt.into_use(&series, &state).render(&state))
        .collect_view();

    // Overlay
    let overlay = overlay
        .into_iter()
        .map(|opt| opt.render(series.clone(), &state))
        .collect_view();

    view! {
        <svg
            width=move || format!("{}px", outer_bounds.get().width())
            height=move || format!("{}px", outer_bounds.get().height())
            viewBox=move || with!(|outer_bounds| format!("0 0 {} {}", outer_bounds.width(), outer_bounds.height()))
            style="display: block; overflow: visible;">
            {inner}
            <DebugRect label="Chart" debug=debug bounds=vec![outer_bounds, outer_bounds] />
            {edges}
            <Series series=series projection=state.projection />
        </svg>
        {overlay}
    }
}
