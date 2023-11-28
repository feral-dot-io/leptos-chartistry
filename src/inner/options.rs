use crate::{series::UseSeries, state::State};
use leptos::*;
use std::rc::Rc;

pub trait InnerLayout<X, Y> {
    fn into_use(self: Rc<Self>, series: &UseSeries<X, Y>, state: &State) -> Box<dyn UseInner>;
}

pub trait UseInner {
    fn render(self: Box<Self>, state: &State) -> View;
}
