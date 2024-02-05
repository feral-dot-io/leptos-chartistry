mod compose;
pub mod legend;
pub mod rotated_label;
pub mod tick_labels;

pub use compose::Layout;

use crate::{
    bounds::Bounds,
    edge::Edge,
    state::{PreState, State},
    Tick,
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

impl<Tick: crate::Tick> EdgeLayout<Tick> {
    fn fixed_height<Y>(&self, state: &PreState<Tick, Y>) -> Signal<f64> {
        match self {
            Self::Legend(inner) => inner.fixed_height(state),
            Self::RotatedLabel(inner) => inner.fixed_height(state),
            Self::TickLabels(inner) => inner.fixed_height(state),
        }
    }
}

impl<X: Tick> EdgeLayout<X> {
    fn to_horizontal_use<Y>(&self, state: &PreState<X, Y>, avail_width: Memo<f64>) -> UseLayout {
        match self {
            Self::Legend(inner) => inner.to_horizontal_use(),
            Self::RotatedLabel(inner) => inner.to_horizontal_use(),
            Self::TickLabels(inner) => inner.to_horizontal_use(state, avail_width),
        }
    }
}

impl<Y: Tick> EdgeLayout<Y> {
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

pub trait IntoEdge<X> {
    fn into_edge(self) -> EdgeLayout<X>;
}

macro_rules! impl_into_edge {
    ($ty:ty, $enum:ident) => {
        impl<V> IntoEdge<V> for $ty {
            fn into_edge(self) -> EdgeLayout<V> {
                EdgeLayout::$enum(self)
            }
        }

        impl<V> From<$ty> for EdgeLayout<V> {
            fn from(inner: $ty) -> Self {
                inner.into_edge()
            }
        }

        impl<V> From<$ty> for Vec<EdgeLayout<V>> {
            fn from(inner: $ty) -> Self {
                vec![inner.into_edge()]
            }
        }
    };
}
impl_into_edge!(legend::Legend, Legend);
impl_into_edge!(rotated_label::RotatedLabel, RotatedLabel);
impl_into_edge!(tick_labels::TickLabels<V>, TickLabels);
