use crate::{
    bounds::Bounds,
    chart::Attr,
    edge::{Edge, IntoEdgeBounds},
    projection::Projection,
    series::UseSeries,
};
use leptos::*;
use std::rc::Rc;

// Note:
// Vertical (left, right, y-axis) options are generated at layout time (constrains the layout)
// Horizontal (top, bottom, x-axis) options are generated at render time (constrained by layout)

pub trait HorizontalLayout<X, Y> {
    fn apply_attr(self, attr: &Attr) -> Rc<dyn HorizontalOption<X, Y>>;
}

pub trait VerticalLayout<X, Y> {
    fn apply_attr(self, attr: &Attr) -> Rc<dyn VerticalOption<X, Y>>;
}

pub trait HorizontalOption<X, Y> {
    fn height(&self) -> Signal<f64>;
    fn into_use(
        self: Rc<Self>,
        series: &UseSeries<X, Y>,
        inner_width: Signal<f64>,
    ) -> Box<dyn UseLayout>;
}

pub trait VerticalOption<X, Y> {
    fn into_use(
        self: Rc<Self>,
        series: &UseSeries<X, Y>,
        inner_height: Signal<f64>,
    ) -> Box<dyn UseLayout>;
}

pub trait UseLayout {
    fn width(&self) -> Signal<f64>;
    fn render(&self, edge: Edge, bounds: Bounds, proj: Signal<Projection>) -> View;
}

type Horizontal<X, Y> = (Rc<dyn HorizontalOption<X, Y>>, Edge, Signal<f64>);

pub struct UnconstrainedLayout<X, Y> {
    horizontal: Vec<Horizontal<X, Y>>,
    pub top_height: Signal<f64>,
    pub bottom_height: Signal<f64>,
}

pub struct ConstrainedLayout<X, Y> {
    horizontal: Vec<Horizontal<X, Y>>,
    pub top_height: Signal<f64>,
    pub bottom_height: Signal<f64>,

    vertical: Vec<(Box<dyn UseLayout>, Edge, Signal<f64>)>,
    pub left_width: Signal<f64>,
    pub right_width: Signal<f64>,
}

#[derive(Clone)]
pub struct ComposedLayout {
    pub projection: Signal<Projection>,
    pub view: Signal<View>,
}

impl<X, Y> UnconstrainedLayout<X, Y> {
    pub fn horizontal_options(
        top: Vec<Rc<dyn HorizontalOption<X, Y>>>,
        mut bottom: Vec<Rc<dyn HorizontalOption<X, Y>>>,
    ) -> Self {
        // Think of layout as top to bottom rather than a stack that goes inwards
        bottom.reverse();

        let mk_horizontal = |edge| {
            move |opt: Rc<dyn HorizontalOption<X, Y>>| {
                let height = opt.height();
                (opt, edge, height)
            }
        };
        let horizontal = top
            .into_iter()
            .map(mk_horizontal(Edge::Top))
            .chain(bottom.into_iter().map(mk_horizontal(Edge::Bottom)))
            .collect::<Vec<_>>();

        // Sizes
        let top_height = option_size_sum(Edge::Top, &horizontal);
        let bottom_height = option_size_sum(Edge::Bottom, &horizontal);

        Self {
            horizontal,
            top_height,
            bottom_height,
        }
    }

    pub fn vertical_options(
        self,
        left: Vec<Rc<dyn VerticalOption<X, Y>>>,
        mut right: Vec<Rc<dyn VerticalOption<X, Y>>>,
        series: &UseSeries<X, Y>,
        inner_height: Signal<f64>,
    ) -> ConstrainedLayout<X, Y> {
        // Compoose left to right
        right.reverse();

        let mk_vertical = |edge| {
            move |opt: Rc<dyn VerticalOption<X, Y>>| {
                let c = opt.into_use(series, inner_height);
                let width = c.width();
                (c, edge, width)
            }
        };
        let vertical = left
            .into_iter()
            .map(mk_vertical(Edge::Left))
            .chain(right.into_iter().map(mk_vertical(Edge::Right)))
            .collect::<Vec<_>>();

        let left_width = option_size_sum(Edge::Left, &vertical);
        let right_width = option_size_sum(Edge::Right, &vertical);

        ConstrainedLayout {
            horizontal: self.horizontal,
            top_height: self.top_height,
            bottom_height: self.bottom_height,

            vertical,
            left_width,
            right_width,
        }
    }
}

impl<X, Y> ConstrainedLayout<X, Y> {
    pub fn compose(
        self,
        outer_bounds: Signal<Bounds>,
        inner_width: Signal<f64>,
        series: &UseSeries<X, Y>,
    ) -> ComposedLayout {
        let options = self
            .horizontal
            .into_iter()
            .map(|(opt, edge, height)| (opt.into_use(series, inner_width), edge, height))
            .chain(self.vertical)
            .collect::<Vec<_>>();

        // Inner chart
        let inner_bounds = Signal::derive(move || {
            outer_bounds.get().shrink(
                self.top_height.get(),
                self.right_width.get(),
                self.bottom_height.get(),
                self.left_width.get(),
            )
        });
        let data = series.data;
        let projection = create_memo(move |_| {
            Projection::new(inner_bounds.get(), data.with(|data| data.position_range()))
        })
        .into();

        // Top / bottom options to UseLayoutOption
        let view = Signal::derive(move || {
            options
                .iter()
                .map(|(opt, edge, size)| (opt, *edge, size.get()))
                .into_edge_bounds(outer_bounds.get(), inner_bounds.get())
                .map(|(c, edge, bounds)| c.render(edge, bounds, projection))
                .collect_view()
        });

        ComposedLayout { projection, view }
    }
}

fn option_size_sum<Opt>(edge: Edge, options: &[(Opt, Edge, Signal<f64>)]) -> Signal<f64> {
    let sizes = options
        .iter()
        .filter(|(_, e, _)| *e == edge)
        .map(|&(_, _, size)| size)
        .collect::<Vec<_>>();
    Signal::derive(move || sizes.iter().map(|size| size.get()).sum::<f64>())
}

impl<T, X, Y> HorizontalLayout<X, Y> for &T
where
    T: Clone + HorizontalLayout<X, Y>,
{
    fn apply_attr(self, attr: &Attr) -> Rc<dyn HorizontalOption<X, Y>> {
        self.clone().apply_attr(attr)
    }
}

impl<T, X, Y> VerticalLayout<X, Y> for &T
where
    T: Clone + VerticalLayout<X, Y>,
{
    fn apply_attr(self, attr: &Attr) -> Rc<dyn VerticalOption<X, Y>> {
        self.clone().apply_attr(attr)
    }
}
