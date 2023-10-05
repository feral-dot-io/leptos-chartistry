use crate::{
    bounds::Bounds,
    chart::Attr,
    edge::{Edge, IntoEdgeBounds},
    RotatedLabel,
};
use leptos::*;

#[derive(Clone, Debug)]
pub enum LayoutOption {
    RotatedLabel(RotatedLabel),
}

impl LayoutOption {
    fn horizontal_size(&self, attr: &Attr) -> Signal<f64> {
        match self {
            Self::RotatedLabel(config) => config.size(attr),
        }
    }

    fn vertical_size(&self, attr: &Attr, avail_height: Signal<f64>) -> Signal<f64> {
        match self {
            Self::RotatedLabel(config) => config.size(attr),
        }
    }
}

fn with_block_size(
    edge: Edge,
    opts: Vec<LayoutOption>,
    sizer: impl Fn(&LayoutOption) -> Signal<f64>,
) -> (Vec<(LayoutOption, Edge, Signal<f64>)>, Signal<f64>) {
    let opts = (opts.into_iter())
        .map(|opt| {
            let size = sizer(&opt);
            (opt, edge, size)
        })
        .collect::<Vec<_>>();
    let size_opts = opts.clone();
    let total_size =
        Signal::derive(move || size_opts.iter().map(|(_, _, size)| size.get()).sum::<f64>());
    (opts, total_size)
}

impl LayoutOption {
    pub fn compose(
        outer_bounds: Signal<Bounds>,
        attr: Attr,
        top: Vec<LayoutOption>,
        right: Vec<LayoutOption>,
        bottom: Vec<LayoutOption>,
        left: Vec<LayoutOption>,
    ) -> impl IntoView {
        // Note:
        // Vertical (left, right, y-axis) options are generated at layout time (constrains the layout)
        // Horizontal (top, bottom, x-axis) options are generated at render time (constrained by layout)

        // Top / bottom options
        let horizontal_sizer = |opt: &LayoutOption| opt.horizontal_size(&attr);
        let (top, top_height) = with_block_size(Edge::Top, top, horizontal_sizer);
        let (bottom, bottom_height) = with_block_size(Edge::Bottom, bottom, horizontal_sizer);
        let avail_height = move || {
            with!(
                |outer_bounds, top_height, bottom_height| outer_bounds.height()
                    - top_height
                    - bottom_height
            )
        };

        // Left / right options (requires height)
        let vertical_sizer = |opt: &LayoutOption| opt.vertical_size(&attr, avail_height.into());
        let (left, left_width) = with_block_size(Edge::Left, left, vertical_sizer);
        let (right, right_width) = with_block_size(Edge::Right, right, vertical_sizer);
        let avail_width = move || {
            with!(|outer_bounds, left_width, right_width| {
                outer_bounds.width() - left_width - right_width
            })
        };

        // Inner chart
        let inner_bounds = Signal::derive(move || {
            outer_bounds.get().shrink(
                top_height.get(),
                right_width.get(),
                bottom_height.get(),
                left_width.get(),
            )
        });

        // Composed layout
        Signal::derive(move || {
            (top.iter())
                .chain(bottom.iter())
                .chain(left.iter())
                .chain(right.iter())
                .map(|(opt, edge, size)| (opt.clone(), *edge, size.get())) // Undo & and reactive
                .into_edge_bounds(outer_bounds.get(), inner_bounds.get())
                .map(|(opt, edge, bounds)| view!(<Layout layout=opt attr=&attr edge=edge bounds=bounds />))
                .collect_view()
        })
    }
}

#[component]
pub fn Layout<'a>(
    layout: LayoutOption,
    attr: &'a Attr,
    edge: Edge,
    bounds: Bounds,
) -> impl IntoView {
    match layout {
        LayoutOption::RotatedLabel(config) => {
            view! { <RotatedLabel config=config attr=attr edge=edge bounds=bounds /> }.into_view()
        }
    }
}
