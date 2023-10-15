use crate::{chart::Attr, projection::Projection, series::UseSeries};
use leptos::*;

pub trait OverlayLayout<X, Y> {
    fn apply_attr(self, attr: &Attr) -> Box<dyn UseOverlay<X, Y>>;
}

pub trait UseOverlay<X, Y> {
    fn render(
        self: Box<Self>,
        series: UseSeries<X, Y>,
        proj: Signal<Projection>,
        mouse_abs: Signal<Option<(f64, f64)>>,
        mouse_rel: Signal<Option<(f64, f64)>>,
    ) -> View;
}

/// Clone references
impl<T, X, Y> OverlayLayout<X, Y> for &T
where
    T: Clone + OverlayLayout<X, Y>,
{
    fn apply_attr(self, attr: &Attr) -> Box<dyn UseOverlay<X, Y>> {
        self.clone().apply_attr(attr)
    }
}
