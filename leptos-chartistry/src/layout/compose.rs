use super::{EdgeLayout, UseLayout};
use crate::{
    aspect_ratio::KnownAspectRatio,
    bounds::Bounds,
    edge::Edge,
    state::{PreState, State},
    Tick,
};
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct Layout {
    pub outer: Memo<Bounds>,
    pub top: Vec<Memo<Bounds>>,
    pub top_bounds: Memo<Bounds>,
    pub right: Vec<Memo<Bounds>>,
    pub right_bounds: Memo<Bounds>,
    pub bottom: Vec<Memo<Bounds>>,
    pub bottom_bounds: Memo<Bounds>,
    pub left: Vec<Memo<Bounds>>,
    pub left_bounds: Memo<Bounds>,
    pub inner: Memo<Bounds>,
    pub x_width: Memo<f64>,
}

#[derive(Clone)]
pub struct DeferredRender {
    edge: Edge,
    bounds: Memo<Bounds>,
    layout: UseLayout,
}

impl DeferredRender {
    pub fn render<X: Clone, Y: Clone>(self, state: State<X, Y>) -> View {
        self.layout.render(self.edge, self.bounds, state)
    }
}

impl Layout {
    /// Composes a layout giving bounds to edges and invididual components.
    ///
    /// Note:
    /// Horizontal (top, bottom, x-axis) options have a fixed height (not dependent on the bounds of other elements) that constrains the layout.
    /// Vertical (left, right, y-axis) options have a variable width and are generated at layout time having been constrained by the horizontal options.
    ///
    /// This function is long but procedural. General process:
    ///  - Constrain the layout using fixed height from top / bottom edges.
    ///  - Calculate the inner height.
    ///  - Process the left / right components using inner height.
    ///  - Calculate the inner width.
    ///  - Process top / bottom components using inner width.
    ///  - Calculate the bounds: outer, inner, edges, edge components. Adhere to aspect ratio.
    ///  - Return state (Layout) and a deferred renderer (ComposedLayout).
    ///
    pub fn compose<X: Tick, Y: Tick>(
        top: &[EdgeLayout<X>],
        right: &[EdgeLayout<Y>],
        bottom: &[EdgeLayout<X>],
        left: &[EdgeLayout<Y>],
        aspect_ratio: Memo<KnownAspectRatio>,
        state: &PreState<X, Y>,
    ) -> (Layout, Vec<DeferredRender>) {
        // Horizontal options
        let top_heights = collect_heights(top, state);
        let top_height = sum_sizes(top_heights.clone());
        let bottom_heights = collect_heights(bottom, state);
        let bottom_height = sum_sizes(bottom_heights.clone());
        let inner_height =
            KnownAspectRatio::inner_height_signal(aspect_ratio, top_height, bottom_height);

        // Vertical options
        let (left_widths, left) = use_vertical(left, state, inner_height);
        let left_width = sum_sizes(left_widths.clone());
        let (right_widths, right) = use_vertical(right, state, inner_height);
        let right_width = sum_sizes(right_widths.clone());
        let avail_width =
            KnownAspectRatio::inner_width_signal(aspect_ratio, left_width, right_width);

        // Bounds
        let outer = create_memo(move |_| {
            Bounds::new(
                left_width.get() + avail_width.get() + right_width.get(),
                top_height.get() + inner_height.get() + bottom_height.get(),
            )
        });
        let inner = create_memo(move |_| {
            outer.get().shrink(
                top_height.get(),
                right_width.get(),
                bottom_height.get(),
                left_width.get(),
            )
        });

        // Edge bounds
        let top_bounds = create_memo(move |_| {
            let i = inner.get();
            Bounds::from_points(i.left_x(), outer.get().top_y(), i.right_x(), i.top_y())
        });
        let right_bounds = create_memo(move |_| {
            let i = inner.get();
            Bounds::from_points(i.right_x(), i.top_y(), outer.get().right_x(), i.bottom_y())
        });
        let bottom_bounds = create_memo(move |_| {
            let i = inner.get();
            let bottom_y = outer.get().bottom_y();
            Bounds::from_points(i.left_x(), i.bottom_y(), i.right_x(), bottom_y)
        });
        let left_bounds = create_memo(move |_| {
            let i = inner.get();
            Bounds::from_points(outer.get().left_x(), i.top_y(), i.left_x(), i.bottom_y())
        });

        // Find the width of each X
        let data_len = state.data.len;
        let x_width = create_memo(move |_| inner.get().width() / data_len.get() as f64);

        // State signals
        let layout = Layout {
            outer,
            top: option_bounds(Edge::Top, top_bounds, top_heights),
            top_bounds,
            right: option_bounds(Edge::Right, right_bounds, right_widths),
            right_bounds,
            bottom: option_bounds(Edge::Bottom, bottom_bounds, bottom_heights),
            bottom_bounds,
            left: option_bounds(Edge::Left, left_bounds, left_widths),
            left_bounds,
            inner,
            x_width,
        };

        let vertical = |edge, bounds: &[Memo<Bounds>], items: Vec<_>| {
            items
                .into_iter()
                .enumerate()
                .map(move |(index, opt)| (edge, bounds[index], opt))
                .collect::<Vec<_>>()
        };
        let horizontal = |edge: Edge, bounds: &[Memo<Bounds>], items: &[EdgeLayout<X>]| {
            items
                .iter()
                .enumerate()
                .map(|(index, opt)| {
                    (
                        edge,
                        bounds[index],
                        opt.to_horizontal_use(state, avail_width),
                    )
                })
                .collect::<Vec<_>>()
        };

        // Chain edges together for a deferred render
        let deferred = vertical(Edge::Left, &layout.left, left)
            .into_iter()
            .chain(vertical(Edge::Right, &layout.right, right))
            .chain(horizontal(Edge::Top, &layout.top, top))
            .chain(horizontal(Edge::Bottom, &layout.bottom, bottom))
            .map(|(edge, bounds, layout)| DeferredRender {
                edge,
                bounds,
                layout,
            })
            .collect::<Vec<_>>();

        (layout, deferred)
    }
}

fn collect_heights<X: Tick, Y>(
    items: &[EdgeLayout<X>],
    state: &PreState<X, Y>,
) -> Vec<Signal<f64>> {
    items
        .iter()
        .map(|c| c.fixed_height(state))
        .collect::<Vec<_>>()
}

fn use_vertical<X: Tick, Y: Tick>(
    items: &[EdgeLayout<Y>],
    state: &PreState<X, Y>,
    avail_height: Memo<f64>,
) -> (Vec<Signal<f64>>, Vec<UseLayout>) {
    items
        .iter()
        .map(|c| {
            let vert = c.to_vertical_use(state, avail_height);
            (vert.width, vert.layout)
        })
        .unzip()
}

fn sum_sizes(sizes: Vec<Signal<f64>>) -> Memo<f64> {
    create_memo(move |_| sizes.iter().map(|opt| opt.get()).sum::<f64>())
}

fn option_bounds(edge: Edge, outer: Memo<Bounds>, sizes: Vec<Signal<f64>>) -> Vec<Memo<Bounds>> {
    let mut seen = Vec::<Signal<f64>>::with_capacity(sizes.len());
    sizes
        .into_iter()
        .map(|size| {
            let prev = seen.clone();
            seen.push(size);
            create_memo(move |_| {
                // Proximal "nearest" and distal "furthest" are distances from the inner edge
                let proximal = prev.iter().map(|s| s.get()).sum::<f64>();
                let distal = proximal + size.get();
                let outer = outer.get();
                let width = outer.width();
                let height = outer.height();
                match edge {
                    Edge::Top => outer.shrink(height - distal, 0.0, proximal, 0.0),
                    Edge::Bottom => outer.shrink(proximal, 0.0, height - distal, 0.0),
                    Edge::Left => outer.shrink(0.0, proximal, 0.0, width - distal),
                    Edge::Right => outer.shrink(0.0, width - distal, 0.0, proximal),
                }
            })
        })
        .collect::<Vec<_>>()
}
