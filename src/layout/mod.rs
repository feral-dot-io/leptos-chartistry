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
pub enum HorizontalLayout<X> {
    Legend(legend::Legend),
    RotatedLabel(rotated_label::RotatedLabel),
    TickLabels(tick_labels::TickLabels<X>),
}

#[derive(Clone)]
#[non_exhaustive]
pub enum VerticalLayout<Y> {
    Legend(legend::Legend),
    RotatedLabel(rotated_label::RotatedLabel),
    TickLabels(tick_labels::TickLabels<Y>),
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

impl<X: PartialEq> HorizontalLayout<X> {
    fn fixed_height<Y>(&self, state: &PreState<X, Y>) -> Signal<f64> {
        match self {
            Self::Legend(inner) => inner.fixed_height(state),
            Self::RotatedLabel(inner) => inner.fixed_height(state),
            Self::TickLabels(inner) => inner.fixed_height(state),
        }
    }

    fn to_use<Y>(&self, state: &PreState<X, Y>, avail_width: Memo<f64>) -> UseLayout {
        match self {
            Self::Legend(inner) => inner.to_horizontal_use(),
            Self::RotatedLabel(inner) => inner.to_horizontal_use(),
            Self::TickLabels(inner) => inner.to_horizontal_use(state, avail_width),
        }
    }
}

impl<Y: PartialEq> VerticalLayout<Y> {
    fn to_use<X>(&self, state: &PreState<X, Y>, avail_height: Memo<f64>) -> UseVerticalLayout {
        match self {
            Self::Legend(inner) => inner.to_vertical_use(state),
            Self::RotatedLabel(inner) => inner.to_vertical_use(state),
            Self::TickLabels(inner) => inner.to_vertical_use(state, avail_height),
        }
    }
}

// TODO: use macros to reduce boilerplate

/// Conversion to a HorizontalLayout
pub trait ToHorizontal<X> {
    fn to_horizontal(&self) -> HorizontalLayout<X>;
}

impl<X> ToHorizontal<X> for legend::Legend {
    fn to_horizontal(&self) -> HorizontalLayout<X> {
        HorizontalLayout::Legend(self.clone())
    }
}

impl<X> ToHorizontal<X> for rotated_label::RotatedLabel {
    fn to_horizontal(&self) -> HorizontalLayout<X> {
        HorizontalLayout::RotatedLabel(self.clone())
    }
}

impl<X: Clone> ToHorizontal<X> for tick_labels::TickLabels<X> {
    fn to_horizontal(&self) -> HorizontalLayout<X> {
        HorizontalLayout::TickLabels(self.clone())
    }
}

impl<T: Clone + ToHorizontal<X>, X> ToHorizontal<X> for &T {
    fn to_horizontal(&self) -> HorizontalLayout<X> {
        (*self).clone().to_horizontal()
    }
}

impl<X: Clone> ToHorizontal<X> for HorizontalLayout<X> {
    fn to_horizontal(&self) -> HorizontalLayout<X> {
        self.clone()
    }
}

/// Conversion to a VerticalLayout
pub trait ToVertical<Y> {
    fn to_vertical(&self) -> VerticalLayout<Y>;
}

impl<Y> ToVertical<Y> for legend::Legend {
    fn to_vertical(&self) -> VerticalLayout<Y> {
        VerticalLayout::Legend(self.clone())
    }
}

impl<Y> ToVertical<Y> for rotated_label::RotatedLabel {
    fn to_vertical(&self) -> VerticalLayout<Y> {
        VerticalLayout::RotatedLabel(self.clone())
    }
}

impl<Y: Clone> ToVertical<Y> for tick_labels::TickLabels<Y> {
    fn to_vertical(&self) -> VerticalLayout<Y> {
        VerticalLayout::TickLabels(self.clone())
    }
}

impl<T: Clone + ToVertical<Y>, Y> ToVertical<Y> for &T {
    fn to_vertical(&self) -> VerticalLayout<Y> {
        (*self).clone().to_vertical()
    }
}

impl<Y: Clone> ToVertical<Y> for VerticalLayout<Y> {
    fn to_vertical(&self) -> VerticalLayout<Y> {
        self.clone()
    }
}

impl<X> From<legend::Legend> for HorizontalLayout<X> {
    fn from(legend: legend::Legend) -> Self {
        Self::Legend(legend)
    }
}

impl<X> From<rotated_label::RotatedLabel> for HorizontalLayout<X> {
    fn from(label: rotated_label::RotatedLabel) -> Self {
        Self::RotatedLabel(label)
    }
}

impl<X> From<tick_labels::TickLabels<X>> for HorizontalLayout<X> {
    fn from(ticks: tick_labels::TickLabels<X>) -> Self {
        Self::TickLabels(ticks)
    }
}

impl<Y> From<legend::Legend> for VerticalLayout<Y> {
    fn from(legend: legend::Legend) -> Self {
        Self::Legend(legend)
    }
}

impl<Y> From<rotated_label::RotatedLabel> for VerticalLayout<Y> {
    fn from(label: rotated_label::RotatedLabel) -> Self {
        Self::RotatedLabel(label)
    }
}

impl<Y> From<tick_labels::TickLabels<Y>> for VerticalLayout<Y> {
    fn from(ticks: tick_labels::TickLabels<Y>) -> Self {
        Self::TickLabels(ticks)
    }
}

/// Wrapper around `Vec<HorizontalLayout<X>>` to facilitate API conversion.
///
/// Use `Default` to create an empty vector, then `push` items to it, and finally convert to a `Vec<VerticalLayout<Y>>` with `Into`.
pub struct HorizontalVec<X>(Vec<HorizontalLayout<X>>);

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

    pub(crate) fn as_slice(&self) -> &[HorizontalLayout<X>] {
        &self.0
    }
}

impl<X> From<HorizontalVec<X>> for Vec<HorizontalLayout<X>> {
    fn from(vec: HorizontalVec<X>) -> Self {
        vec.0
    }
}

impl<T: ToHorizontal<X>, X> From<Vec<T>> for HorizontalVec<X> {
    fn from(items: Vec<T>) -> Self {
        Self(
            items
                .into_iter()
                .map(|i| i.to_horizontal())
                .collect::<Vec<_>>(),
        )
    }
}

/// Wrapper around `Vec<VerticalLayout<Y>>` to facilitate API conversion.
///
/// Use `Default` to create an empty vector, then `push` items to it, and finally convert to a `Vec<VerticalLayout<Y>>` with `Into`.
pub struct VerticalVec<Y>(Vec<VerticalLayout<Y>>);

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

    pub(crate) fn as_slice(&self) -> &[VerticalLayout<Y>] {
        &self.0
    }
}

/// End result. Convert to a `Vec<VerticalLayout<Y>>`
impl<Y> From<VerticalVec<Y>> for Vec<VerticalLayout<Y>> {
    fn from(vec: VerticalVec<Y>) -> Self {
        vec.0
    }
}

impl<T: ToVertical<Y>, Y> From<Vec<T>> for VerticalVec<Y> {
    fn from(items: Vec<T>) -> Self {
        Self(
            items
                .into_iter()
                .map(|i| i.to_vertical())
                .collect::<Vec<_>>(),
        )
    }
}
