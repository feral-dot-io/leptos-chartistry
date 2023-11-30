use crate::{
    aspect_ratio::{AspectRatioCalc, CalcUsing},
    debug::DebugRect,
    inner::InnerLayout,
    layout::{HorizontalLayout, Layout, VerticalLayout},
    overlay::OverlayLayout,
    projection::Projection,
    series::{Series, UseSeries},
    state::{PreState, State},
    use_watched_node::{use_watched_node, UseWatchedNode},
    AspectRatio, Font, Padding,
};
use leptos::{html::Div, *};
use std::rc::Rc;

#[derive(Clone)]
pub struct Chart<X: 'static, Y: 'static> {
    debug: Signal<bool>,
    font: Signal<Font>,
    padding: Signal<Padding>,

    top: Vec<Rc<dyn HorizontalLayout<X, Y>>>,
    right: Vec<Rc<dyn VerticalLayout<X, Y>>>,
    bottom: Vec<Rc<dyn HorizontalLayout<X, Y>>>,
    left: Vec<Rc<dyn VerticalLayout<X, Y>>>,
    inner: Vec<Rc<dyn InnerLayout<X, Y>>>,
    overlay: Vec<Rc<dyn OverlayLayout<X, Y>>>,
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
            debug: debug.into(),
            font: font.into(),
            padding: padding.into(),

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

    pub fn inner(mut self, opt: impl InnerLayout<X, Y> + 'static) -> Self {
        self.inner.push(Rc::new(opt));
        self
    }

    pub fn overlay(mut self, opt: impl OverlayLayout<X, Y> + 'static) -> Self {
        self.overlay.push(Rc::new(opt));
        self
    }
}

#[component]
pub fn Chart<X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static>(
    chart: Chart<X, Y>,
    #[prop(into)] aspect_ratio: MaybeSignal<AspectRatio>,
) -> impl IntoView {
    let root = create_node_ref::<Div>();
    let watch = use_watched_node(root);

    // Aspect ratio signal
    let have_dimensions = create_memo(move |_| watch.bounds.get().is_some());
    let width = create_memo(move |_| watch.bounds.get().unwrap_or_default().width());
    let height = create_memo(move |_| watch.bounds.get().unwrap_or_default().height());
    let calc = create_memo(move |_| match aspect_ratio.get().0 {
        CalcUsing::Env(calc) => calc.mk_signal(width, height),
        CalcUsing::Known(calc) => calc,
    });

    let debug = chart.debug;
    view! {
        <div node_ref=root style="width: fit-content; height: fit-content; overflow: visible;">
            <DebugRect label="Chart" debug=debug />
            <Show when=move || have_dimensions.get() fallback=|| view!(<p>"Loading..."</p>)>
                <RenderChart
                    chart=chart.clone()
                    watch=watch.clone()
                    aspect_ratio=calc
                />
            </Show>
        </div>
    }
}

#[component]
fn RenderChart<X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static>(
    chart: Chart<X, Y>,
    watch: UseWatchedNode,
    aspect_ratio: Memo<AspectRatioCalc>,
) -> impl IntoView {
    let Chart {
        debug,
        font,
        padding,

        mut top,
        right,
        bottom,
        mut left,
        inner,
        overlay,
        series,
    } = chart;

    // Edges are added top to bottom, left to right. Layout compoeses inside out:
    top.reverse();
    left.reverse();

    // Compose edges
    let pre = PreState::new(debug, font, padding, series.clone());
    let (layout, edges) = Layout::compose(top, right, bottom, left, aspect_ratio, &pre);

    // Finalise state
    let projection = {
        let inner = layout.inner;
        let data = series.data;
        create_memo(move |_| Projection::new(inner.get(), data.with(|data| data.position_range())))
            .into()
    };
    let state = State::new(pre, &watch, layout, projection);

    // Render edges
    let edges = edges.into_iter().map(|r| r.render(&state)).collect_view();

    // Inner
    let inner = inner
        .into_iter()
        .map(|opt| opt.into_use(&state).render(&state))
        .collect_view();

    // Overlay
    let overlay = overlay
        .into_iter()
        .map(|opt| opt.render(&state))
        .collect_view();

    let outer = state.layout.outer;
    view! {
        <svg
            width=move || format!("{}px", outer.get().width())
            height=move || format!("{}px", outer.get().height())
            viewBox=move || with!(|outer| format!("0 0 {} {}", outer.width(), outer.height()))
            style="display: block; overflow: visible;">
            <DebugRect label="RenderChart" debug=debug bounds=vec![outer.into()] />
            {inner}
            {edges}
            <Series series=series projection=state.projection />
        </svg>
        {overlay}
    }
}
