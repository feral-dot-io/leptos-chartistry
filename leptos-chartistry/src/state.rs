use crate::{
    layout::Layout, projection::Projection, series::UseData, use_watched_node::UseWatchedNode,
    Padding, Tick,
};
use leptos::prelude::*;

#[derive(Clone)]
#[non_exhaustive]
pub struct PreState<X: Tick, Y: Tick> {
    pub debug: Signal<bool>,
    pub font_height: Memo<f64>,
    pub font_width: Memo<f64>,
    pub padding: Signal<Padding>,
    pub data: UseData<X, Y>,
}

#[derive(Clone)]
#[non_exhaustive]
pub struct State<X: Tick, Y: Tick> {
    pub pre: PreState<X, Y>,
    pub layout: Layout,
    pub projection: Signal<Projection>,

    pub svg_zero: Memo<(f64, f64)>,

    /// Mouse page position
    pub mouse_page: Signal<(f64, f64)>,
    /// Mouse page position relative to chart
    pub mouse_chart: Signal<(f64, f64)>,
    /// Mouse over inner chart?
    pub hover_inner: Signal<bool>,
    /// X mouse coord in data position space
    pub hover_position_x: Memo<f64>,
}

impl<X: Tick, Y: Tick> PreState<X, Y> {
    pub fn new(
        debug: Signal<bool>,
        font_height: Memo<f64>,
        font_width: Memo<f64>,
        padding: Signal<Padding>,
        data: UseData<X, Y>,
    ) -> Self {
        Self {
            debug,
            font_height,
            font_width,
            padding,
            data,
        }
    }
}

impl<X: Tick, Y: Tick> State<X, Y> {
    pub fn new(
        pre: PreState<X, Y>,
        node: &UseWatchedNode,
        layout: Layout,
        proj: Signal<Projection>,
    ) -> Self {
        // Mouse
        let mouse_chart = node.mouse_chart;
        let hover_inner = node.mouse_hover_inner(layout.inner);

        // Data
        let hover_position = Memo::new(move |_| {
            let (mouse_x, mouse_y) = mouse_chart.get();
            proj.get().svg_to_position(mouse_x, mouse_y)
        });
        let hover_position_x = Memo::new(move |_| hover_position.get().0);

        Self {
            pre,
            layout,
            projection: proj,
            svg_zero: Memo::new(move |_| proj.get().position_to_svg(0.0, 0.0)),

            mouse_page: node.mouse_page,
            mouse_chart,
            hover_inner,
            hover_position_x,
        }
    }
}
