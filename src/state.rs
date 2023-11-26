use crate::{bounds::Bounds, projection::Projection, use_watched_node::UseWatchedNode};
use leptos::signal_prelude::*;

#[derive(Clone, Debug)]
pub struct State {
    pub projection: Signal<Projection>,
    /// Size of chart (left and top are 0)
    pub bounds: Signal<Option<Bounds>>,
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
    pub fn new(projection: Signal<Projection>, watched_node: &UseWatchedNode) -> Self {
        Self {
            projection,
            bounds: watched_node.bounds,
            mouse_page: watched_node.mouse_page,
            mouse_chart: watched_node.mouse_chart,
            mouse_hover_chart: watched_node.mouse_chart_hover,
            mouse_hover_inner: watched_node.mouse_hover_inner(projection),
        }
    }
}
