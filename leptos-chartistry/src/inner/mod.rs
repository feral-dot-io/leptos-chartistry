pub mod axis_marker;
pub mod grid_line;
pub mod guide_line;
pub mod legend;

use crate::{state::State, Tick};
use axis_marker::AxisMarker;
use grid_line::{XGridLine, YGridLine};
use guide_line::{XGuideLine, YGuideLine};
use legend::InsetLegend;
use leptos::prelude::*;

/// Inner layout options for a [Chart](crate::Chart). See [IntoInner](trait@IntoInner) for details.
#[derive(Clone)]
#[non_exhaustive]
pub enum InnerLayout<X: Tick, Y: Tick> {
    /// Axis marker. See [AxisMarker](struct@axis_marker::AxisMarker) for details.
    AxisMarker(axis_marker::AxisMarker),
    /// X grid line. See [XGridLine](struct@grid_line::XGridLine) for details.
    XGridLine(grid_line::XGridLine<X>),
    /// Y grid line. See [YGridLine](struct@grid_line::YGridLine) for details.
    YGridLine(grid_line::YGridLine<Y>),
    /// X guide line. See [XGuideLine](struct@guide_line::XGuideLine) for details.
    XGuideLine(guide_line::XGuideLine),
    /// Y guide line. See [YGuideLine](struct@guide_line::YGuideLine) for details.
    YGuideLine(guide_line::YGuideLine),
    /// Inset legend. See [InsetLegend](struct@legend::InsetLegend) for details.
    Legend(legend::InsetLegend),
}

/// Convert a type (e.g., a [guide line](struct@guide_line::XGuideLine)) into an inner layout for use in a [Chart](crate::Chart).
pub trait IntoInner<X: Tick, Y: Tick> {
    /// Create an inner layout from the type. See [IntoInner](trait@IntoInner) for details.
    fn into_inner(self) -> InnerLayout<X, Y>;
}

pub enum UseInner<X: Tick, Y: Tick> {
    AxisMarker(axis_marker::AxisMarker),
    XGridLine(grid_line::UseXGridLine<X>),
    YGridLine(grid_line::UseYGridLine<Y>),
    XGuideLine(guide_line::UseXGuideLine),
    YGuideLine(guide_line::UseYGuideLine),
    Legend(legend::InsetLegend),
}

impl<X: Tick, Y: Tick> InnerLayout<X, Y> {
    pub(super) fn into_use(self, state: &State<X, Y>) -> UseInner<X, Y> {
        match self {
            Self::AxisMarker(inner) => UseInner::AxisMarker(inner),
            Self::XGridLine(inner) => UseInner::XGridLine(inner.use_horizontal(state)),
            Self::YGridLine(inner) => UseInner::YGridLine(inner.use_vertical(state)),
            Self::XGuideLine(inner) => UseInner::XGuideLine(inner.use_horizontal()),
            Self::YGuideLine(inner) => UseInner::YGuideLine(inner.use_vertical()),
            Self::Legend(inner) => UseInner::Legend(inner),
        }
    }
}

impl<X: Tick, Y: Tick> UseInner<X, Y> {
    pub(super) fn render(self, state: State<X, Y>) -> impl IntoView {
        match self {
            Self::AxisMarker(inner) => view! {<AxisMarker marker=inner state=state />}.into_any(),
            Self::XGridLine(inner) => view! {<XGridLine line=inner state=state />}.into_any(),
            Self::YGridLine(inner) => view! {<YGridLine line=inner state=state />}.into_any(),
            Self::XGuideLine(inner) => view! {<XGuideLine line=inner state=state />}.into_any(),
            Self::YGuideLine(inner) => view! {<YGuideLine line=inner state=state />}.into_any(),
            Self::Legend(inner) => view! {<InsetLegend legend=inner state=state />}.into_any(),
        }
    }
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
