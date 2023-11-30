use super::{InnerLayout, UseInner};
use crate::{edge::Edge, state::State, Anchor, Legend, Snippet};
use leptos::*;
use std::{borrow::Borrow, rc::Rc};

#[derive(Clone, Debug)]
pub struct InsetLegend {
    edge: Edge,
    legend: Legend,
}

impl InsetLegend {
    fn new(edge: Edge, anchor: Anchor, snippet: impl Borrow<Snippet>) -> Self {
        Self {
            edge,
            legend: Legend::new(anchor, *snippet.borrow()),
        }
    }

    pub fn top_left(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Top, Anchor::Start, snippet)
    }
    pub fn top(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Top, Anchor::Middle, snippet)
    }
    pub fn top_right(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Top, Anchor::End, snippet)
    }
    pub fn bottom_left(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Bottom, Anchor::Start, snippet)
    }
    pub fn bottom(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Bottom, Anchor::Middle, snippet)
    }
    pub fn bottom_right(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Bottom, Anchor::End, snippet)
    }
    pub fn left(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Left, Anchor::Middle, snippet)
    }
    pub fn right(snippet: impl Borrow<Snippet>) -> Self {
        Self::new(Edge::Right, Anchor::Middle, snippet)
    }
}

impl<X, Y> InnerLayout<X, Y> for InsetLegend {
    fn into_use(self: Rc<Self>, _: &State<X, Y>) -> Box<dyn UseInner<X, Y>> {
        Box::new((*self).clone())
    }
}

impl<X, Y> UseInner<X, Y> for InsetLegend {
    fn render(self: Box<Self>, state: &State<X, Y>) -> View {
        view!( <InsetLegend legend=self.legend edge=self.edge state=state /> )
    }
}

#[component]
fn InsetLegend<'a, X: 'static, Y: 'static>(
    legend: Legend,
    edge: Edge,
    state: &'a State<X, Y>,
) -> impl IntoView {
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
