use crate::{
    chart::Attr, projection::Projection, series::UseSeries, use_watched_node::UseWatchedNode,
};
use leptos::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

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
    fn render(self: Box<Self>, proj: Signal<Projection>, watch: &UseWatchedNode) -> View;
}

/// Clone references
impl<T, X, Y> InnerLayout<X, Y> for &T
where
    T: Clone + InnerLayout<X, Y>,
{
    fn apply_attr(self, attr: &Attr) -> Box<dyn InnerOption<X, Y>> {
        self.clone().apply_attr(attr)
    }
}

/// Passthru option to use
impl<T, X, Y> InnerOption<X, Y> for T
where
    T: UseInner + 'static,
{
    fn to_use(self: Box<Self>, _: &UseSeries<X, Y>, _: Signal<Projection>) -> Box<dyn UseInner> {
        self
    }
}

impl std::fmt::Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Axis::Horizontal => write!(f, "horizontal"),
            Axis::Vertical => write!(f, "vertical"),
        }
    }
}
