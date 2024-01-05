pub mod axis_marker;
pub mod grid_line;
pub mod guide_line;
pub mod legend;

use crate::state::State;
use leptos::*;
use std::rc::Rc;

#[derive(Clone)]
pub enum InnerLayout<X: Clone, Y: Clone> {
    AxisMarker(axis_marker::AxisMarker),
    HorizontalGridLine(grid_line::GridLine<X>),
    VerticalGridLine(grid_line::GridLine<Y>),
    // TODO: promote GuideLine::Axis into this enum
    GuideLine(guide_line::GuideLine),
    Legend(legend::InsetLegend),
}

impl<X: Clone + PartialEq, Y: Clone + PartialEq> InnerLayout<X, Y> {
    pub fn into_use(self, state: &State<X, Y>) -> Rc<dyn UseInner<X, Y>> {
        match self {
            Self::AxisMarker(inner) => Rc::new(inner),
            Self::HorizontalGridLine(inner) => inner.use_horizontal(state),
            Self::VerticalGridLine(inner) => inner.use_vertical(state),
            Self::GuideLine(inner) => Rc::new(inner),
            Self::Legend(inner) => Rc::new(inner),
        }
    }
}

pub trait UseInner<X, Y> {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View;
}
