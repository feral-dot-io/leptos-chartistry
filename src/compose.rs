use super::{legend::Legend, rotated_label::RotatedLabel, tick_labels::TickLabels};
use crate::{
    bounds::Bounds,
    chart::Attr,
    edge::{Edge, IntoEdgeBounds},
    projection::Projection,
    series::UseSeries,
};
use leptos::*;

pub enum LayoutOption<Tick: 'static> {
    RotatedLabel(RotatedLabel),
    Legend(Legend),
    TickLabels(TickLabels<Tick>),
}

pub trait HorizontalOption<X, Y> {
    fn height(&self) -> Signal<f64>;
    fn to_use(
        self: Box<Self>,
        series: &UseSeries<X, Y>,
        avail_width: Signal<f64>,
    ) -> Box<dyn UseLayout>;
}

pub trait VerticalOption<X, Y> {
    fn to_use(
        self: Box<Self>,
        series: &UseSeries<X, Y>,
        avail_height: Signal<f64>,
    ) -> Box<dyn UseLayout>;
}

pub trait UseLayout {
    fn width(&self) -> Signal<f64>;
    fn render<'a>(&self, edge: Edge, bounds: Bounds, proj: Signal<Projection>) -> View;
}

#[derive(Clone, Debug)]
pub struct Layout {
    pub projection: Signal<Projection>,
    pub view: Signal<View>,
}

impl Layout {
    pub fn compose<X, Y>(
        outer_bounds: Signal<Bounds>,
        top: Vec<Box<dyn HorizontalOption<X, Y>>>,
        right: Vec<Box<dyn VerticalOption<X, Y>>>,
        bottom: Vec<Box<dyn HorizontalOption<X, Y>>>,
        left: Vec<Box<dyn VerticalOption<X, Y>>>,
        series: &UseSeries<X, Y>,
    ) -> Layout {
        // Note:
        // Vertical (left, right, y-axis) options are generated at layout time (constrains the layout)
        // Horizontal (top, bottom, x-axis) options are generated at render time (constrained by layout)

        // Top / bottom heights
        let top_heights = (top.iter()).map(|opt| opt.height()).collect::<Vec<_>>();
        let bottom_heights = (bottom.iter()).map(|opt| opt.height()).collect::<Vec<_>>();
        let horiz_height = |heights: Vec<Signal<f64>>| {
            Signal::derive(move || (heights.iter()).map(|h| h.get()).sum::<f64>())
        };
        let top_height = horiz_height(top_heights.clone());
        let bottom_height = horiz_height(bottom_heights.clone());
        let avail_height = Signal::derive(move || {
            with!(
                |outer_bounds, top_height, bottom_height| outer_bounds.height()
                    - top_height
                    - bottom_height
            )
        });

        // Left / right options to UseLayoutOption
        let to_vertical = |opts: Vec<Box<dyn VerticalOption<X, Y>>>, edge: Edge| {
            (opts.into_iter())
                .map(|opt| {
                    let c = opt.to_use(series, avail_height);
                    let width = c.width();
                    (c, edge, width)
                })
                .collect::<Vec<_>>()
        };
        let left = to_vertical(left, Edge::Left);
        let right = to_vertical(right, Edge::Right);

        // Left / right widths
        let vert_width = |widths: &[(_, _, Signal<f64>)]| {
            let widths = widths.iter().map(|(_, _, w)| *w).collect::<Vec<_>>();
            Signal::derive(move || (widths.iter()).map(|w| w.get()).sum::<f64>())
        };
        let left_width = vert_width(&left);
        let right_width = vert_width(&right);
        let avail_width = Signal::derive(move || {
            outer_bounds.get().width() - left_width.get() - right_width.get()
        });

        // Convert top / bottom to UseLayout
        let horizontal = (top.into_iter())
            .zip(top_heights.into_iter())
            .map(|(opt, height)| (opt, Edge::Top, height))
            .chain(
                (bottom.into_iter())
                    .zip(bottom_heights.into_iter())
                    .map(|(opt, height)| (opt, Edge::Bottom, height)),
            )
            .map(|(opt, edge, height)| (opt.to_use(series, avail_width), edge, height))
            .collect::<Vec<_>>();

        // Inner chart
        let inner_bounds = Signal::derive(move || {
            outer_bounds.get().shrink(
                top_height.get(),
                right_width.get(),
                bottom_height.get(),
                left_width.get(),
            )
        });
        let data = series.data;
        let projection = Signal::derive(move || {
            Projection::new(inner_bounds.get(), data.with(|data| data.position_range()))
        });

        // Top / bottom options to UseLayoutOption
        let view = Signal::derive(move || {
            (horizontal.iter())
                .chain(left.iter().chain(right.iter()))
                .map(|(opt, edge, size)| (opt, *edge, size.get()))
                .into_edge_bounds(outer_bounds.get(), inner_bounds.get())
                .map(|(c, edge, bounds)| c.render(edge, bounds, projection))
                .collect_view()
        });

        Self { projection, view }
    }
}

impl<X> LayoutOption<X> {
    pub(crate) fn apply_horizontal<Y: 'static>(
        self,
        attr: &Attr,
    ) -> Box<dyn HorizontalOption<X, Y>> {
        match self {
            Self::RotatedLabel(config) => Box::new(config.apply_horizontal(attr)),
            Self::Legend(config) => Box::new(config.apply_horizontal(attr)),
            Self::TickLabels(config) => Box::new(config.apply_horizontal(attr)),
        }
    }
}

impl<Y> LayoutOption<Y> {
    pub(crate) fn apply_vertical<X: 'static>(self, attr: &Attr) -> Box<dyn VerticalOption<X, Y>> {
        match self {
            Self::RotatedLabel(config) => Box::new(config.apply_vertical(attr)),
            Self::Legend(config) => Box::new(config.apply_vertical(attr)),
            Self::TickLabels(config) => Box::new(config.apply_vertical(attr)),
        }
    }
}