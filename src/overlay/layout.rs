use crate::{chart::Attr, series::UseSeries, state::State};
use leptos::*;
use std::rc::Rc;

pub trait OverlayLayout<X, Y> {
    fn apply_attr(self, attr: &Attr) -> Rc<dyn UseOverlay<X, Y>>;
}

pub trait UseOverlay<X, Y> {
    fn render(self: Rc<Self>, series: UseSeries<X, Y>, state: &State) -> View;
}

/// Clone references
impl<T, X, Y> OverlayLayout<X, Y> for &T
where
    T: Clone + OverlayLayout<X, Y>,
{
    fn apply_attr(self, attr: &Attr) -> Rc<dyn UseOverlay<X, Y>> {
        self.clone().apply_attr(attr)
    }
}
