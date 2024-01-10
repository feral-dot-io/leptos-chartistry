pub mod axis_marker;
pub mod grid_line;
pub mod guide_line;
pub mod legend;

use crate::state::State;
use leptos::*;
use std::rc::Rc;

// The colours for our layout can be specified (overridden) on each individual component. If no colour is specified then we fall back to the layout colour scheme. This default three palette scheme from darker to lighter grey. Below are the indexes for each colour in the scheme giving that hierarchy to the component.
const LAYOUT_GUIDE_LINE: usize = 0;
const LAYOUT_AXIS_MARKER: usize = 1;
const LAYOUT_GRID_LINE: usize = 2;

#[derive(Clone)]
pub enum InnerLayout<X: Clone, Y: Clone> {
    AxisMarker(axis_marker::AxisMarker),
    HorizontalGridLine(grid_line::GridLine<X>),
    VerticalGridLine(grid_line::GridLine<Y>),
    XGuideLine(guide_line::GuideLine),
    YGuideLine(guide_line::GuideLine),
    Legend(legend::InsetLegend),
}

impl<X: Clone + PartialEq, Y: Clone + PartialEq> InnerLayout<X, Y> {
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
