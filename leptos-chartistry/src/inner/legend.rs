use super::UseInner;
use crate::{edge::Edge, state::State, Anchor, Legend};
use leptos::prelude::*;
use std::rc::Rc;

/// Builds an inset legend for the chart [series](crate::Series). Differs from [Legend](struct@Legend) by being placed inside the chart area.
#[derive(Clone, Debug)]
pub struct InsetLegend {
    /// Edge of the chart area to place the legend.
    pub edge: RwSignal<Edge>,
    /// Legend to display. Relies on the internal `anchor` signal. See [Legend](struct@Legend) for details.
    pub legend: Legend,
}

impl InsetLegend {
    fn new(edge: Edge, anchor: Anchor) -> Self {
        Self {
            edge: create_rw_signal(edge),
            legend: Legend::new(anchor),
        }
    }

    /// Creates a new inset legend placed at the top-left corner of the chart area.
    pub fn top_left() -> Self {
        Self::new(Edge::Top, Anchor::Start)
    }
    /// Creates a new inset legend placed at the top-middle of the chart area.
    pub fn top() -> Self {
        Self::new(Edge::Top, Anchor::Middle)
    }
    /// Creates a new inset legend placed at the top-right corner of the chart area.
    pub fn top_right() -> Self {
        Self::new(Edge::Top, Anchor::End)
    }
    /// Creates a new inset legend placed at the bottom-left corner of the chart area.
    pub fn bottom_left() -> Self {
        Self::new(Edge::Bottom, Anchor::Start)
    }
    /// Creates a new inset legend placed at the bottom-middle of the chart area.
    pub fn bottom() -> Self {
        Self::new(Edge::Bottom, Anchor::Middle)
    }
    /// Creates a new inset legend placed at the bottom-right corner of the chart area.
    pub fn bottom_right() -> Self {
        Self::new(Edge::Bottom, Anchor::End)
    }
    /// Creates a new inset legend placed at the left-middle of the chart area.
    pub fn left() -> Self {
        Self::new(Edge::Left, Anchor::Middle)
    }
    /// Creates a new inset legend placed at the right-middle of the chart area.
    pub fn right() -> Self {
        Self::new(Edge::Right, Anchor::Middle)
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
        let (top, right, bottom, left) = match edge.get() {
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
