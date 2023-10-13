use crate::{chart::Attr, projection::Projection, series::UseSeries};
use leptos::*;

pub trait InnerLayout<X, Y> {
    fn apply_attr(self, attr: &Attr) -> Box<dyn InnerOption<X, Y>>;
}

pub trait InnerOption<X, Y> {
    fn to_use(
        self: Box<Self>,
        series: &UseSeries<X, Y>,
        proj: Signal<Projection>,
    ) -> Box<dyn UseInner>;
}

pub trait UseInner {
    fn render(self: Box<Self>, proj: Signal<Projection>) -> View;
}

impl<T, X, Y> InnerLayout<X, Y> for &T
where
    T: Clone + InnerLayout<X, Y>,
{
    fn apply_attr(self, attr: &Attr) -> Box<dyn InnerOption<X, Y>> {
        self.clone().apply_attr(attr)
    }
}
