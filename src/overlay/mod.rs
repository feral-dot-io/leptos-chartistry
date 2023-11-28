pub mod tooltip;

use crate::{series::UseSeries, state::State};
use leptos::*;
use std::rc::Rc;

pub trait OverlayLayout<X, Y> {
    fn render(self: Rc<Self>, series: UseSeries<X, Y>, state: &State) -> View;
}
