pub mod tooltip;

use crate::state::State;
use leptos::*;
use std::rc::Rc;

pub trait OverlayLayout<X, Y> {
    fn render(self: Rc<Self>, state: &State<X, Y>) -> View;
}
