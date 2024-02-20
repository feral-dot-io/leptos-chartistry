use crate::{
    bounds::Bounds, layout::Layout, projection::Projection, series::UseData,
    use_watched_node::UseWatchedNode, Padding, Tick,
};
use leptos::signal_prelude::*;

#[derive(Clone)]
pub struct PreState<X: 'static, Y: 'static> {
    pub debug: Signal<bool>,
    pub font_height: Memo<f64>,
    pub font_width: Memo<f64>,
    pub padding: Signal<Padding>,
    pub data: UseData<X, Y>,
}

#[derive(Clone)]
pub struct State<X: 'static, Y: 'static> {
    pub pre: PreState<X, Y>,
    pub layout: Layout,
    pub projection: Signal<Projection>,

    pub svg_zero: Memo<(f64, f64)>,

    /// Size of chart on page (left and top are 0)
    pub page_bounds: Signal<Option<Bounds>>,
    /// Mouse page position
    pub mouse_page: Signal<(f64, f64)>,
    /// Mouse page position relative to chart
    pub mouse_chart: Signal<(f64, f64)>,
    /// Mouse over chart?
    pub hover_chart: Signal<bool>,
    /// Mouse over inner chart?
    pub hover_inner: Signal<bool>,
    /// X mouse coord in data position space
    pub hover_position_x: Memo<f64>,
    /// Y mouse coord in data position space
    pub hover_position_y: Memo<f64>,

    /// X coord of nearest mouse data in SVG space
    pub nearest_svg_x: Memo<Option<f64>>,
}

impl<X, Y> PreState<X, Y> {
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
        let hover_position = create_memo(move |_| {
            let (mouse_x, mouse_y) = mouse_chart.get();
            proj.get().svg_to_position(mouse_x, mouse_y)
        });
        let hover_position_x = create_memo(move |_| hover_position.get().0);
        let hover_position_y = create_memo(move |_| hover_position.get().1);

        let nearest_pos_x = pre.data.nearest_aligned_position_x(hover_position_x);
        let nearest_svg_x = create_memo(move |_| {
            nearest_pos_x.get().map(|pos_x| {
                let (svg_x, _) = proj.get().position_to_svg(pos_x, 0.0);
                svg_x
            })
        });

        Self {
            pre,
            layout,
            projection: proj,
            svg_zero: create_memo(move |_| proj.get().position_to_svg(0.0, 0.0)),

            page_bounds: node.bounds,
            mouse_page: node.mouse_page,
            mouse_chart,
            hover_chart: node.mouse_chart_hover,
            hover_inner,
            hover_position_x,
            hover_position_y,

            nearest_svg_x,
        }
    }
}
