use crate::{
    series::UseSeries,
    state::{AttrState, State},
};
use leptos::*;
use std::rc::Rc;

pub trait OverlayLayout<X, Y> {
    fn into_use(self, attr: &AttrState) -> Rc<dyn UseOverlay<X, Y>>;
}

pub trait UseOverlay<X, Y> {
    fn render(self: Rc<Self>, series: UseSeries<X, Y>, state: &State) -> View;
}

/// Clone references
impl<T, X, Y> OverlayLayout<X, Y> for &T
where
    T: Clone + OverlayLayout<X, Y>,
{
    fn into_use(self, attr: &AttrState) -> Rc<dyn UseOverlay<X, Y>> {
        self.clone().into_use(attr)
    }
}
