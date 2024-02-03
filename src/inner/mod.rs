pub mod axis_marker;
pub mod grid_line;
pub mod guide_line;
pub mod legend;

use crate::{state::State, Tick};
use leptos::*;
use std::rc::Rc;

#[derive(Clone)]
#[non_exhaustive]
pub enum InnerLayout<X: Tick, Y: Tick> {
    AxisMarker(axis_marker::AxisMarker),
    XGridLine(grid_line::XGridLine<X>),
    YGridLine(grid_line::YGridLine<Y>),
    XGuideLine(guide_line::XGuideLine),
    YGuideLine(guide_line::YGuideLine),
    Legend(legend::InsetLegend),
}

pub trait IntoInnerLayout<X: Tick, Y: Tick> {
    fn into_inner_layout(self) -> InnerLayout<X, Y>;
}

impl<X: Tick, Y: Tick> InnerLayout<X, Y> {
    pub fn into_use(self, state: &State<X, Y>) -> Rc<dyn UseInner<X, Y>> {
        match self {
            Self::AxisMarker(inner) => Rc::new(inner),
            Self::XGridLine(inner) => inner.use_horizontal(state),
            Self::YGridLine(inner) => inner.use_vertical(state),
            Self::XGuideLine(inner) => inner.use_horizontal(),
            Self::YGuideLine(inner) => inner.use_vertical(),
            Self::Legend(inner) => Rc::new(inner),
        }
    }
}

pub trait UseInner<X, Y> {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View;
}

macro_rules! impl_into_inner_layout {
    ($ty:ty, $enum:ident) => {
        impl<X: Tick, Y: Tick> IntoInnerLayout<X, Y> for $ty {
            fn into_inner_layout(self) -> InnerLayout<X, Y> {
                InnerLayout::$enum(self)
            }
        }

        impl<X: Tick, Y: Tick> From<$ty> for InnerLayout<X, Y> {
            fn from(inner: $ty) -> Self {
                inner.into_inner_layout()
            }
        }
    };
}
impl_into_inner_layout!(axis_marker::AxisMarker, AxisMarker);
impl_into_inner_layout!(grid_line::XGridLine<X>, XGridLine);
impl_into_inner_layout!(grid_line::YGridLine<Y>, YGridLine);
impl_into_inner_layout!(guide_line::XGuideLine, XGuideLine);
impl_into_inner_layout!(guide_line::YGuideLine, YGuideLine);
impl_into_inner_layout!(legend::InsetLegend, Legend);
