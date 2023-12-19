use super::{InnerLayout, UseInner};
use crate::{edge::Edge, state::State, Anchor, Legend};
use leptos::*;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct InsetLegend {
    edge: Edge,
    legend: Legend,
}

impl InsetLegend {
    fn new(edge: Edge, anchor: Anchor) -> Self {
        Self {
            edge,
            legend: Legend::new(anchor),
        }
    }

    pub fn top_left() -> Self {
        Self::new(Edge::Top, Anchor::Start)
    }
    pub fn top() -> Self {
        Self::new(Edge::Top, Anchor::Middle)
    }
    pub fn top_right() -> Self {
        Self::new(Edge::Top, Anchor::End)
    }
    pub fn bottom_left() -> Self {
        Self::new(Edge::Bottom, Anchor::Start)
    }
    pub fn bottom() -> Self {
        Self::new(Edge::Bottom, Anchor::Middle)
    }
    pub fn bottom_right() -> Self {
        Self::new(Edge::Bottom, Anchor::End)
    }
    pub fn left() -> Self {
        Self::new(Edge::Left, Anchor::Middle)
    }
    pub fn right() -> Self {
        Self::new(Edge::Right, Anchor::Middle)
    }
}

impl<X: Clone, Y: Clone> InnerLayout<X, Y> for InsetLegend {
    fn into_use(self: Rc<Self>, _: &State<X, Y>) -> Rc<dyn UseInner<X, Y>> {
        self
    }
}

impl<X: Clone, Y: Clone> UseInner<X, Y> for InsetLegend {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <InsetLegend legend=(*self).clone() state=state /> )
    }
}

#[component]
fn InsetLegend<X: Clone + 'static, Y: Clone + 'static>(
    legend: InsetLegend,
    state: State<X, Y>,
) -> impl IntoView {
    let InsetLegend { edge, legend } = legend;
    let inner = state.layout.inner;
    let width = Legend::width(&state.pre);
    let height = legend.fixed_height(&state.pre);
    let bounds = create_memo(move |_| {
        let inner = inner.get();
        let height = height.get();
        let width = width.get();
        // Build legend bounds as an inset of the chart bounds
        let (top, right, bottom, left) = match edge {
            Edge::Top => (0.0, 0.0, inner.height() - height, 0.0),
            Edge::Bottom => (inner.height() - height, 0.0, 0.0, 0.0),
            Edge::Left => (0.0, inner.width() - width, 0.0, 0.0),
            Edge::Right => (0.0, 0.0, 0.0, inner.width() - width),
        };
        inner.shrink(top, right, bottom, left)
    });

    view! {
        <g class="_chartistry_legend_inset">
            <Legend legend=legend edge=edge bounds=bounds state=state />
        </g>
    }
}
