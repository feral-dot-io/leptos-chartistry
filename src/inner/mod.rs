pub mod axis_marker;
pub mod grid_line;
pub mod guide_line;
pub mod legend;

use crate::state::State;
use leptos::*;
use std::rc::Rc;

pub trait InnerLayout<X, Y> {
    fn into_use(self: Rc<Self>, state: &State<X, Y>) -> Rc<dyn UseInner<X, Y>>;
}

pub trait UseInner<X, Y> {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View;
}
