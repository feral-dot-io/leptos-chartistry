use crate::{
    bounds::Bounds, layout::Layout, projection::Projection, use_watched_node::UseWatchedNode, Font,
    Padding,
};
use leptos::signal_prelude::*;

#[derive(Clone, Debug)]
pub struct AttrState {
    pub debug: Signal<bool>,
    pub font: Signal<Font>,
    pub padding: Signal<Padding>,
}

#[derive(Clone, Debug)]
pub struct State {
    pub attr: AttrState,
    pub layout: Layout,
    pub projection: Signal<Projection>,

    pub svg_zero: Memo<(f64, f64)>,

    /// Size of chart on page (left and top are 0)
    pub page_bounds: Signal<Option<Bounds>>,
    /// Mouse position in page coords
    pub mouse_page: Signal<(f64, f64)>,
    /// Mouse position relative to bounds in page coords
    pub mouse_chart: Signal<(f64, f64)>,
    /// Mouse over chart?
    pub mouse_hover_chart: Signal<bool>,
    /// Mouse over inner chart?
    pub mouse_hover_inner: Signal<bool>,
}

impl State {
    pub fn new(
        attr: AttrState,
        layout: Layout,
        proj: Signal<Projection>,
        watched_node: &UseWatchedNode,
    ) -> Self {
        let mouse_hover_inner = watched_node.mouse_hover_inner(layout.inner);
        Self {
            attr,
            layout,
            projection: proj,
            svg_zero: create_memo(move |_| proj.get().data_to_svg(0.0, 0.0)),
            page_bounds: watched_node.bounds,
            mouse_page: watched_node.mouse_page,
            mouse_chart: watched_node.mouse_chart,
            mouse_hover_chart: watched_node.mouse_chart_hover,
            mouse_hover_inner,
        }
    }
}
