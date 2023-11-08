use crate::{
    chart::Attr, projection::Projection, series::UseSeries, use_watched_node::UseWatchedNode,
};
use leptos::*;
use std::rc::Rc;

pub trait InnerLayout<X, Y> {
    fn apply_attr(self, attr: &Attr) -> Rc<dyn InnerOption<X, Y>>;
}

pub trait InnerOption<X, Y> {
    fn to_use(
        self: Rc<Self>,
        series: &UseSeries<X, Y>,
        proj: Signal<Projection>,
    ) -> Box<dyn UseInner>;
}

pub trait UseInner {
    fn render(self: Box<Self>, proj: Signal<Projection>, watch: &UseWatchedNode) -> View;
}

/// Clone references
impl<T, X, Y> InnerLayout<X, Y> for &T
where
    T: Clone + InnerLayout<X, Y>,
{
    fn apply_attr(self, attr: &Attr) -> Rc<dyn InnerOption<X, Y>> {
        self.clone().apply_attr(attr)
    }
}

/// Passthru option to use
impl<T, X, Y> InnerOption<X, Y> for T
where
    T: Clone + UseInner + 'static,
{
    fn to_use(self: Rc<Self>, _: &UseSeries<X, Y>, _: Signal<Projection>) -> Box<dyn UseInner> {
        Box::new((*self).clone())
    }
}
