pub mod axis_marker;
pub mod grid_line;
pub mod guide_line;
pub mod legend;

use crate::{state::State, Tick};
use leptos::prelude::*;
use std::rc::Rc;

/// Inner layout options for a [Chart](crate::Chart). See [IntoInner](trait@IntoInner) for details.
///
/// Avoid constructing directly.
#[derive(Clone)]
#[doc(hidden)]
#[non_exhaustive]
pub enum InnerLayout<X: Tick, Y: Tick> {
    /// Axis marker. See [AxisMarker](axis_marker::AxisMarker) for details.
    AxisMarker(axis_marker::AxisMarker),
    /// X grid line. See [XGridLine](grid_line::XGridLine) for details.
    XGridLine(grid_line::XGridLine<X>),
    /// Y grid line. See [YGridLine](grid_line::YGridLine) for details.
    YGridLine(grid_line::YGridLine<Y>),
    /// X guide line. See [XGuideLine](guide_line::XGuideLine) for details.
    XGuideLine(guide_line::XGuideLine),
    /// Y guide line. See [YGuideLine](guide_line::YGuideLine) for details.
    YGuideLine(guide_line::YGuideLine),
    /// Inset legend. See [InsetLegend](legend::InsetLegend) for details.
    Legend(legend::InsetLegend),
}

/// Convert a type (e.g., a [guide line](struct@guide_line::XGuideLine)) into an inner layout for use in a [Chart](crate::Chart).
pub trait IntoInner<X: Tick, Y: Tick> {
    /// Create an inner layout from the type. See [IntoInner](trait@IntoInner) for details.
    fn into_inner(self) -> InnerLayout<X, Y>;
}

impl<X: Tick, Y: Tick> InnerLayout<X, Y> {
    pub(super) fn into_use(self, state: &State<X, Y>) -> Rc<dyn UseInner<X, Y>> {
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

macro_rules! impl_into_inner {
    ($ty:ty, $enum:ident) => {
        impl<X: Tick, Y: Tick> IntoInner<X, Y> for $ty {
            fn into_inner(self) -> InnerLayout<X, Y> {
                InnerLayout::$enum(self)
            }
        }

        impl<X: Tick, Y: Tick> From<$ty> for InnerLayout<X, Y> {
            fn from(inner: $ty) -> Self {
                inner.into_inner()
            }
        }

        impl<X: Tick, Y: Tick> From<$ty> for Vec<InnerLayout<X, Y>> {
            fn from(inner: $ty) -> Self {
                vec![inner.into_inner()]
            }
        }
    };
}
impl_into_inner!(axis_marker::AxisMarker, AxisMarker);
impl_into_inner!(grid_line::XGridLine<X>, XGridLine);
impl_into_inner!(grid_line::YGridLine<Y>, YGridLine);
impl_into_inner!(guide_line::XGuideLine, XGuideLine);
impl_into_inner!(guide_line::YGuideLine, YGuideLine);
impl_into_inner!(legend::InsetLegend, Legend);
