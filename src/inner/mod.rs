pub mod axis_marker;
pub mod grid_line;
pub mod guide_line;
pub mod legend;

use crate::{colours::Colour, state::State, Tick};
use leptos::*;
use std::rc::Rc;

// Fall back colours used if nothing is specified directly on the component. These are from darker to lighter grey.
pub const DEFAULT_COLOUR_GUIDE_LINE: Colour = Colour::new(0x9A, 0x9A, 0x9A); // Light grey
pub const DEFAULT_COLOUR_AXIS_MARKER: Colour = Colour::new(0xD2, 0xD2, 0xD2); // Lighter grey
pub const DEFAULT_COLOUR_GRID_LINE: Colour = Colour::new(0xEF, 0xF2, 0xFA); // Lightest grey

#[derive(Clone)]
pub enum InnerLayout<X: Clone + 'static, Y: Clone + 'static> {
    AxisMarker(axis_marker::AxisMarker),
    HorizontalGridLine(grid_line::HorizontalGridLine<X>),
    VerticalGridLine(grid_line::VerticalGridLine<Y>),
    XGuideLine(guide_line::GuideLine),
    YGuideLine(guide_line::GuideLine),
    Legend(legend::InsetLegend),
}

impl<X: Tick, Y: Tick> InnerLayout<X, Y> {
    pub fn into_use(self, state: &State<X, Y>) -> Rc<dyn UseInner<X, Y>> {
        match self {
            Self::AxisMarker(inner) => Rc::new(inner),
            Self::HorizontalGridLine(inner) => inner.use_horizontal(state),
            Self::VerticalGridLine(inner) => inner.use_vertical(state),
            Self::XGuideLine(inner) => inner.use_x(),
            Self::YGuideLine(inner) => inner.use_y(),
            Self::Legend(inner) => Rc::new(inner),
        }
    }
}

pub trait UseInner<X, Y> {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View;
}

macro_rules! impl_into_inner_layout {
    ($ty:ty, $enum:ident) => {
        impl<X: Tick, Y: Tick> From<$ty> for InnerLayout<X, Y> {
            fn from(inner: $ty) -> Self {
                Self::$enum(inner)
            }
        }
    };
}
impl_into_inner_layout!(axis_marker::AxisMarker, AxisMarker);
impl_into_inner_layout!(grid_line::HorizontalGridLine<X>, HorizontalGridLine);
impl_into_inner_layout!(grid_line::VerticalGridLine<Y>, VerticalGridLine);
//impl_into_inner_layout!(guide_line::GuideLine, XGuideLine);
//impl_into_inner_layout!(guide_line::GuideLine, YGuideLine);
impl_into_inner_layout!(legend::InsetLegend, Legend);
