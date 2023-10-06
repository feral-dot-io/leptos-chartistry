use super::rotated_label::{RotatedLabel, UseRotatedLabel};
use crate::{
    bounds::Bounds,
    chart::Attr,
    edge::{Edge, IntoEdgeBounds},
    projection::Projection,
};
use leptos::*;

#[derive(Clone, Debug)]
pub struct Layout {
    pub projection: Signal<Projection>,
    pub view: Signal<View>,
}

#[derive(Clone, Debug)]
pub enum LayoutOption {
    RotatedLabel(RotatedLabel),
}

#[derive(Clone, Debug)]
enum UseLayoutOption {
    RotatedLabel(UseRotatedLabel),
}

fn with_block_size(
    edge: Edge,
    opts: Vec<LayoutOption>,
    attr: &Attr,
    sizer: impl Fn(&UseLayoutOption) -> Signal<f64>,
) -> (Vec<(UseLayoutOption, Edge, Signal<f64>)>, Signal<f64>) {
    let opts = (opts.into_iter())
        .map(|opt| {
            let opt = opt.to_use(attr);
            let size = sizer(&opt);
            (opt, edge, size)
        })
        .collect::<Vec<_>>();
    let size_opts = opts.clone();
    let total_size =
        Signal::derive(move || size_opts.iter().map(|(_, _, size)| size.get()).sum::<f64>());
    (opts, total_size)
}

impl Layout {
    pub fn compose<'a>(
        outer_bounds: Signal<Bounds>,
        attr: &'a Attr,
        top: Vec<LayoutOption>,
        right: Vec<LayoutOption>,
        bottom: Vec<LayoutOption>,
        left: Vec<LayoutOption>,
    ) -> Layout {
        // Note:
        // Vertical (left, right, y-axis) options are generated at layout time (constrains the layout)
        // Horizontal (top, bottom, x-axis) options are generated at render time (constrained by layout)

        // Top / bottom options
        let horizontal_sizer = |opt: &UseLayoutOption| opt.horizontal_size();
        let (top, top_height) = with_block_size(Edge::Top, top, attr, horizontal_sizer);
        let (bottom, bottom_height) = with_block_size(Edge::Bottom, bottom, attr, horizontal_sizer);
        let avail_height = move || {
            with!(
                |outer_bounds, top_height, bottom_height| outer_bounds.height()
                    - top_height
                    - bottom_height
            )
        };

        // Left / right options (requires height)
        let vertical_sizer = |opt: &UseLayoutOption| opt.vertical_size(avail_height.into());
        let (left, left_width) = with_block_size(Edge::Left, left, attr, vertical_sizer);
        let (right, right_width) = with_block_size(Edge::Right, right, attr, vertical_sizer);
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

        // Compose sides
        let view = Signal::derive(move || {
            (top.iter())
                .chain(bottom.iter())
                .chain(left.iter())
                .chain(right.iter())
                .map(|(opt, edge, size)| (opt.clone(), *edge, size.get())) // Undo & and reactive
                .into_edge_bounds(outer_bounds.get(), inner_bounds.get())
                .map(|(opt, edge, bounds)| opt.view(edge, bounds))
                .collect_view()
        });

        // TODO
        let range = Bounds::from_points(0.0, -1.0, 13.0, 1.0);
        let proj = move || Projection::new(inner_bounds.get(), range);

        Self {
            projection: proj.into(),
            view,
        }
    }
}

impl LayoutOption {
    fn to_use(self, attr: &Attr) -> UseLayoutOption {
        match self {
            Self::RotatedLabel(config) => UseLayoutOption::RotatedLabel(config.to_use(attr)),
        }
    }
}

impl UseLayoutOption {
    fn horizontal_size(&self) -> Signal<f64> {
        match self {
            Self::RotatedLabel(config) => config.size(),
        }
    }

    fn vertical_size(&self, avail_height: Signal<f64>) -> Signal<f64> {
        match self {
            Self::RotatedLabel(config) => config.size(),
        }
    }

    fn view(self, edge: Edge, bounds: Bounds) -> impl IntoView {
        match self {
            Self::RotatedLabel(label) => {
                view! { <RotatedLabel label=label edge=edge bounds=bounds /> }
            }
        }
    }
}
