use crate::{
    series::UseSeries,
    state::{AttrState, State},
};
use leptos::*;
use std::rc::Rc;

pub trait InnerLayout<X, Y> {
    fn apply_attr(self, attr: &AttrState) -> Rc<dyn InnerOption<X, Y>>;
}

pub trait InnerOption<X, Y> {
    fn into_use(self: Rc<Self>, series: &UseSeries<X, Y>, state: &State) -> Box<dyn UseInner>;
}

pub trait UseInner {
    fn render(self: Box<Self>, state: &State) -> View;
}

/// Clone references
impl<T, X, Y> InnerLayout<X, Y> for &T
where
    T: Clone + InnerLayout<X, Y>,
{
    fn apply_attr(self, attr: &AttrState) -> Rc<dyn InnerOption<X, Y>> {
        self.clone().apply_attr(attr)
    }
}

/// Passthru option to use
impl<T, X, Y> InnerOption<X, Y> for T
where
    T: Clone + UseInner + 'static,
{
    fn into_use(self: Rc<Self>, _: &UseSeries<X, Y>, _: &State) -> Box<dyn UseInner> {
        Box::new((*self).clone())
    }
}
