use crate::{bounds::Bounds, edge::Edge, state::State, UseSeries};
use leptos::*;
use std::rc::Rc;

/*
Compose:
    - Start with some form of bounds (either inner or outer)
    - List of top / bottom sizes.
    - Constrains the layout.
    - Calculate the left / right sizes using contraints.
    - Return a composed layout.
*/

#[derive(Clone, Debug)]
pub struct Layout {
    pub options: LayoutOptions,
    pub top_bounds: Signal<Bounds>,
    pub right_bounds: Signal<Bounds>,
    pub bottom_bounds: Signal<Bounds>,
    pub left_bounds: Signal<Bounds>,
    pub inner: Memo<Bounds>,
}

#[derive(Clone, Debug)]
pub struct LayoutOptions {
    pub top: Memo<Vec<Bounds>>,
    pub right: Memo<Vec<Bounds>>,
    pub bottom: Memo<Vec<Bounds>>,
    pub left: Memo<Vec<Bounds>>,
}

pub trait HorizontalOption<X, Y> {
    fn fixed_height(&self) -> Signal<f64>;
    fn into_use(
        self: Rc<Self>,
        series: &UseSeries<X, Y>,
        bounds: Signal<Bounds>,
    ) -> Rc<dyn UseLayout>;
}

pub trait VerticalOption<X, Y> {
    fn into_use(
        self: Rc<Self>,
        series: &UseSeries<X, Y>,
        inner_height: Memo<f64>,
    ) -> (Signal<f64>, Rc<dyn UseLayout>);
}

pub trait UseLayout {
    fn render(&self, edge: Edge, bounds: Signal<Bounds>, state: &State) -> View;
}

pub struct ConstrainedLayout {
    pub top: Vec<Signal<f64>>,
    pub top_height: Signal<f64>,
    pub bottom: Vec<Signal<f64>>,
    pub bottom_height: Signal<f64>,
}

impl ConstrainedLayout {
    // TODO: pass vec of signals?
    // TODO: ensure bottom is reversed prior to this point.
    pub fn new<X, Y>(
        top: Vec<&dyn HorizontalOption<X, Y>>,
        bottom: Vec<&dyn HorizontalOption<X, Y>>,
    ) -> Self {
        let (top, top_height) = Self::collect_heights(&top);
        let (bottom, bottom_height) = Self::collect_heights(&bottom);
        Self {
            top,
            top_height,
            bottom,
            bottom_height,
        }
    }

    fn collect_heights<X, Y>(
        items: &[&dyn HorizontalOption<X, Y>],
    ) -> (Vec<Signal<f64>>, Signal<f64>) {
        let heights = items
            .into_iter()
            .map(|c| c.fixed_height())
            .collect::<Vec<_>>();
        let total = Signal::derive(move || heights.iter().map(|opt| opt.get()).sum::<f64>());
        (heights, total)
    }

    pub fn compose<X, Y>(
        self,
        outer: Signal<Bounds>,
        series: &UseSeries<X, Y>,
        left: Vec<&dyn VerticalOption<X, Y>>,
        right: Vec<&dyn VerticalOption<X, Y>>,
    ) -> Layout {
        let ConstrainedLayout {
            top,
            top_height,
            bottom,
            bottom_height,
        } = self;
        let inner_height =
            create_memo(move |_| outer.get().height() - top_height.get() - bottom_height.get());

        // Vertical options
        let left = left
            .into_iter()
            .map(|c| c.width(series, inner_height))
            .collect::<Vec<_>>();
        let right = right
            .into_iter()
            .map(|c| c.width(series, inner_height))
            .collect::<Vec<_>>();
        let left_width = {
            let left = left.to_owned();
            Signal::derive(move || left.iter().map(|opt| opt.get()).sum::<f64>())
        };
        let right_width = {
            let right = right.to_owned();
            Signal::derive(move || right.iter().map(|opt| opt.get()).sum::<f64>())
        };

        // Inner chart
        let inner = create_memo(move |_| {
            outer.get().shrink(
                top_height.get(),
                right_width.get(),
                bottom_height.get(),
                left_width.get(),
            )
        });

        // Edge bounds
        let top_bounds = Signal::derive(move || {
            let i = inner.get();
            Bounds::from_points(i.left_x(), outer.get().top_y(), i.right_x(), i.top_y())
        });
        let right_bounds = Signal::derive(move || {
            let i = inner.get();
            Bounds::from_points(i.right_x(), i.top_y(), outer.get().right_x(), i.bottom_y())
        });
        let bottom_bounds = Signal::derive(move || {
            let i = inner.get();
            let bottom_y = outer.get().bottom_y();
            Bounds::from_points(i.left_x(), i.bottom_y(), i.right_x(), bottom_y)
        });
        let left_bounds = Signal::derive(move || {
            let i = inner.get();
            Bounds::from_points(outer.get().left_x(), i.top_y(), i.left_x(), i.bottom_y())
        });

        // Option edge bounds
        // TODO: If we run out of space, prioritise options closest to inner
        let options = LayoutOptions {
            top: inner_bounds(Edge::Top, outer, top),
            right: inner_bounds(Edge::Right, outer, right),
            bottom: inner_bounds(Edge::Bottom, outer, bottom),
            left: inner_bounds(Edge::Left, outer, left),
        };

        Layout {
            options,
            top_bounds,
            right_bounds,
            bottom_bounds,
            left_bounds,
            inner,
        }
    }
}

fn inner_bounds(edge: Edge, outer: Signal<Bounds>, sizes: Vec<Signal<f64>>) -> Memo<Vec<Bounds>> {
    create_memo(move |_| {
        let outer = outer.get();
        let width = outer.width();
        let height = outer.height();
        let mut acc = 0.0;
        sizes
            .iter()
            .rev()
            .map(|size| {
                let prev_acc = acc;
                acc += size.get(); // Shrink handles overflow

                match edge {
                    Edge::Top => {
                        outer.shrink(height - acc, outer.right_x(), prev_acc, outer.left_x())
                    }
                    Edge::Bottom => {
                        outer.shrink(prev_acc, outer.right_x(), height - acc, outer.left_x())
                    }
                    Edge::Left => {
                        outer.shrink(outer.top_y(), prev_acc, outer.bottom_y(), width - acc)
                    }
                    Edge::Right => {
                        outer.shrink(outer.top_y(), width - acc, outer.bottom_y(), prev_acc)
                    }
                }
            })
            .collect::<Vec<_>>()
    })
}
