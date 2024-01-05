pub mod axis_marker;
pub mod grid_line;
pub mod guide_line;
pub mod legend;

use crate::state::State;
use leptos::*;
use std::rc::Rc;

pub trait InnerLayout<X, Y>: private::Sealed {
    fn into_use(self: Rc<Self>, state: &State<X, Y>) -> Rc<dyn UseInner<X, Y>>;
}

pub trait UseInner<X, Y> {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View;
}

mod private {
    pub trait Sealed {}
    impl Sealed for super::axis_marker::AxisMarker {}
    impl<X: Clone> Sealed for super::grid_line::HorizontalGridLine<X> {}
    impl<Y: Clone> Sealed for super::grid_line::VerticalGridLine<Y> {}
    impl Sealed for super::guide_line::GuideLine {}
    impl Sealed for super::legend::InsetLegend {}
}
