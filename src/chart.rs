use crate::{
    bounds::Bounds,
    debug::DebugRect,
    inner::{InnerLayout, InnerOption},
    layout::{HorizontalLayout, HorizontalOption, Layout, VerticalLayout, VerticalOption},
    overlay::{OverlayLayout, UseOverlay},
    series::{Series, UseSeries},
    use_watched_node::{use_watched_node, UseWatchedNode},
    Font, Padding,
};
use leptos::{html::Div, *};
use std::rc::Rc;

#[derive(Clone)]
pub struct Chart<X: 'static, Y: 'static> {
    padding: Option<MaybeSignal<Padding>>,
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
            padding: None,
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

    pub fn set_padding(mut self, padding: impl Into<MaybeSignal<Padding>>) -> Self {
        self.padding = Some(padding.into());
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

    pub fn add_top(mut self, opt: impl HorizontalLayout<X, Y>) -> Self {
        self.top.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn add_right(mut self, opt: impl VerticalLayout<X, Y>) -> Self {
        self.right.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn add_bottom(mut self, opt: impl HorizontalLayout<X, Y>) -> Self {
        self.bottom.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn add_left(mut self, opt: impl VerticalLayout<X, Y>) -> Self {
        self.left.push(opt.apply_attr(&self.attr));
        self
    }

    pub fn add(mut self, opt: impl InnerLayout<X, Y>) -> Self {
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
    #[prop(optional, into)] width: MaybeProp<f64>,
    #[prop(optional, into)] height: MaybeProp<f64>,
) -> impl IntoView {
    let root = create_node_ref::<Div>();
    let watch = use_watched_node(root, width, height);

    let render = move || match watch.bounds.get() {
        Some(root_bounds) => view! {
            <RenderChart
                chart=chart.clone()
                watch=watch.clone()
                chart_bounds=Bounds::new(root_bounds.width(), root_bounds.height()) />
        }
        .into_view(),
        None => view!(<p>"Loading..."</p>).into_view(),
    };

    view! {
        <div
            node_ref=root
            style:width=move || width.get().map(|width| format!("{width}px")).unwrap_or("100%".to_string())
            style:height=move || height.get().map(|height| format!("{height}px")).unwrap_or("100%".to_string())
        >
            {render}
        </div>
    }
}

#[component]
fn RenderChart<X: Clone + 'static, Y: Clone + 'static>(
    chart: Chart<X, Y>,
    watch: UseWatchedNode,
    chart_bounds: Bounds,
) -> impl IntoView {
    let Chart {
        padding,
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

    let padding = padding.unwrap_or(attr.padding);
    let debug = debug.unwrap_or(attr.debug);

    // Layout
    let outer_bounds = Signal::derive(move || padding.get().apply(chart_bounds));
    let layout = Layout::compose(outer_bounds, top, right, bottom, left, &series);

    // Inner layout
    let inner = (inner.into_iter())
        .map(|opt| {
            opt.to_use(&series, layout.projection)
                .render(layout.projection, &watch)
        })
        .collect_view();

    // Overlay
    let overlay = (overlay.into_iter())
        .map(|opt| opt.render(series.clone(), layout.projection, &watch))
        .collect_view();

    view! {
        <svg viewBox=move || format!("0 0 {} {}", chart_bounds.width(), chart_bounds.height())>
            {inner}
            <DebugRect label="Chart" debug=debug bounds=move || vec![chart_bounds, outer_bounds.get()] />
            {layout.view}
            <Series series=series projection=layout.projection />
        </svg>
        {overlay}
    }
}
