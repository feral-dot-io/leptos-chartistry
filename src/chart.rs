use crate::{
    aspect_ratio::{AspectRatioCalc, CalcUsing},
    debug::DebugRect,
    inner::InnerLayout,
    layout::{HorizontalLayout, Layout, VerticalLayout},
    overlay::OverlayLayout,
    projection::Projection,
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

        mut top,
        right,
        bottom,
        mut left,
        inner,
        overlay,
        series,
    } = chart;
    let debug = attr.debug;

    // Edges are added top to bottom, left to right. Layout compoeses inside out:
    top.reverse();
    left.reverse();

    // Compose edges
    let (layout, edges) = Layout::compose(top, right, bottom, left, aspect_ratio, &attr, &series);

    // Finalise state
    let projection = {
        let inner = layout.inner;
        let data = series.data;
        create_memo(move |_| Projection::new(inner.get(), data.with(|data| data.position_range())))
            .into()
    };
    let state = State::new(attr, layout, projection, &watch);

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

    let outer = state.layout.outer;
    view! {
        <svg
            width=move || format!("{}px", outer.get().width())
            height=move || format!("{}px", outer.get().height())
            viewBox=move || with!(|outer| format!("0 0 {} {}", outer.width(), outer.height()))
            style="display: block; overflow: visible;"
        >
            <DebugRect label="Chart" debug=debug bounds=vec![outer.into()] />
            {edges.render(&state)}
            {inner}
            <Series series=series projection=state.projection />
        </svg>
        {overlay}
    }
}
