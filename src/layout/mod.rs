mod compose;
pub mod legend;
pub mod rotated_label;
pub mod tick_labels;

pub use compose::Layout;

use crate::{
    bounds::Bounds,
    edge::Edge,
    state::{PreState, State},
};
use leptos::*;

#[derive(Clone)]
#[non_exhaustive]
pub enum EdgeLayout<Tick: 'static> {
    Legend(legend::Legend),
    RotatedLabel(rotated_label::RotatedLabel),
    TickLabels(tick_labels::TickLabels<Tick>),
}

struct UseVerticalLayout {
    width: Signal<f64>,
    layout: UseLayout,
}

#[derive(Clone)]
enum UseLayout {
    Legend(legend::Legend),
    RotatedLabel(rotated_label::RotatedLabel),
    TickLabels(tick_labels::UseTickLabels),
}

impl UseLayout {
    fn render<X: Clone, Y: Clone>(
        self,
        edge: Edge,
        bounds: Memo<Bounds>,
        state: State<X, Y>,
    ) -> View {
        match self {
            Self::Legend(inner) => view! {
                <legend::Legend legend=inner edge=edge bounds=bounds state=state />
            },
            Self::RotatedLabel(inner) => view! {
                <rotated_label::RotatedLabel label=inner edge=edge bounds=bounds state=state />
            },
            Self::TickLabels(inner) => view! {
                <tick_labels::TickLabels ticks=inner edge=edge bounds=bounds state=state />
            },
        }
    }
}

impl<Tick: PartialEq> EdgeLayout<Tick> {
    fn fixed_height<Y>(&self, state: &PreState<Tick, Y>) -> Signal<f64> {
        match self {
            Self::Legend(inner) => inner.fixed_height(state),
            Self::RotatedLabel(inner) => inner.fixed_height(state),
            Self::TickLabels(inner) => inner.fixed_height(state),
        }
    }
}

impl<X: PartialEq> EdgeLayout<X> {
    fn to_horizontal_use<Y>(&self, state: &PreState<X, Y>, avail_width: Memo<f64>) -> UseLayout {
        match self {
            Self::Legend(inner) => inner.to_horizontal_use(),
            Self::RotatedLabel(inner) => inner.to_horizontal_use(),
            Self::TickLabels(inner) => inner.to_horizontal_use(state, avail_width),
        }
    }
}

impl<Y: PartialEq> EdgeLayout<Y> {
    fn to_vertical_use<X>(
        &self,
        state: &PreState<X, Y>,
        avail_height: Memo<f64>,
    ) -> UseVerticalLayout {
        match self {
            Self::Legend(inner) => inner.to_vertical_use(state),
            Self::RotatedLabel(inner) => inner.to_vertical_use(state),
            Self::TickLabels(inner) => inner.to_vertical_use(state, avail_height),
        }
    }
}

// TODO: use macros to reduce boilerplate

/// Conversion to a HorizontalLayout
pub trait ToEdgeLayout<X> {
    fn to_edge_layout(&self) -> EdgeLayout<X>;
}

impl<X> ToEdgeLayout<X> for legend::Legend {
    fn to_edge_layout(&self) -> EdgeLayout<X> {
        EdgeLayout::Legend(self.clone())
    }
}

impl<X> ToEdgeLayout<X> for rotated_label::RotatedLabel {
    fn to_edge_layout(&self) -> EdgeLayout<X> {
        EdgeLayout::RotatedLabel(self.clone())
    }
}

impl<X: Clone> ToEdgeLayout<X> for tick_labels::TickLabels<X> {
    fn to_edge_layout(&self) -> EdgeLayout<X> {
        EdgeLayout::TickLabels(self.clone())
    }
}

impl<T: Clone + ToEdgeLayout<X>, X> ToEdgeLayout<X> for &T {
    fn to_edge_layout(&self) -> EdgeLayout<X> {
        (*self).clone().to_edge_layout()
    }
}

impl<X: Clone> ToEdgeLayout<X> for EdgeLayout<X> {
    fn to_edge_layout(&self) -> EdgeLayout<X> {
        self.clone()
    }
}

impl<X> From<legend::Legend> for EdgeLayout<X> {
    fn from(legend: legend::Legend) -> Self {
        Self::Legend(legend)
    }
}

impl<X> From<rotated_label::RotatedLabel> for EdgeLayout<X> {
    fn from(label: rotated_label::RotatedLabel) -> Self {
        Self::RotatedLabel(label)
    }
}

impl<X> From<tick_labels::TickLabels<X>> for EdgeLayout<X> {
    fn from(ticks: tick_labels::TickLabels<X>) -> Self {
        Self::TickLabels(ticks)
    }
}

#[derive(Clone)]
pub struct HorizontalVec<X: 'static>(Vec<EdgeLayout<X>>);

impl<X> Default for HorizontalVec<X> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

// Note: this interface is minimised to avoid exposing the `Vec` API
impl<X> HorizontalVec<X> {
    pub(crate) fn reverse(&mut self) {
        self.0.reverse();
    }

    pub(crate) fn as_slice(&self) -> &[EdgeLayout<X>] {
        &self.0
    }
}

impl<X> From<HorizontalVec<X>> for Vec<EdgeLayout<X>> {
    fn from(vec: HorizontalVec<X>) -> Self {
        vec.0
    }
}

impl<T: ToEdgeLayout<X>, X> From<Vec<T>> for HorizontalVec<X> {
    fn from(items: Vec<T>) -> Self {
        Self(
            items
                .into_iter()
                .map(|i| i.to_edge_layout())
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Clone)]
pub struct VerticalVec<Y: 'static>(Vec<EdgeLayout<Y>>);

/// Start with an empty vector
impl<Y> Default for VerticalVec<Y> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

/// Add items to the vector. Could use `vec![item.to_vertical()]` instead.
impl<Y> VerticalVec<Y> {
    pub(crate) fn reverse(&mut self) {
        self.0.reverse();
    }

    pub(crate) fn as_slice(&self) -> &[EdgeLayout<Y>] {
        &self.0
    }
}

/// End result. Convert to a `Vec<VerticalLayout<Y>>`
impl<Y> From<VerticalVec<Y>> for Vec<EdgeLayout<Y>> {
    fn from(vec: VerticalVec<Y>) -> Self {
        vec.0
    }
}

impl<T: ToEdgeLayout<Y>, Y> From<Vec<T>> for VerticalVec<Y> {
    fn from(items: Vec<T>) -> Self {
        Self(
            items
                .into_iter()
                .map(|i| i.to_edge_layout())
                .collect::<Vec<_>>(),
        )
    }
}
