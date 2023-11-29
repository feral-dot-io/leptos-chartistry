use crate::{
    bounds::Bounds, layout::Layout, projection::Projection, series,
    use_watched_node::UseWatchedNode, Font, Padding,
};
use leptos::signal_prelude::*;

#[derive(Clone, Debug)]
pub struct AttrState {
    pub debug: Signal<bool>,
    pub font: Signal<Font>,
    pub padding: Signal<Padding>,
}

#[derive(Clone, Debug)]
pub struct Data<X: 'static, Y: 'static> {
    pub x_range: Memo<Option<(X, X)>>,
    pub y_range: Memo<Option<(Y, Y)>>,
}

#[derive(Clone, Debug)]
pub struct PreState<X: 'static, Y: 'static> {
    pub attr: AttrState,
    pub data: Data<X, Y>,
}

#[derive(Clone, Debug)]
pub struct State<X: 'static, Y: 'static> {
    pub attr: AttrState,
    pub data: Data<X, Y>,
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

    // Data
    pub mouse_hover_data: Memo<(f64, f64)>,
    pub mouse_hover_nearest_data_x: Memo<f64>,
    pub mouse_hover_nearest_svg_x: Memo<f64>,
}

impl<X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static> PreState<X, Y> {
    pub fn new(attr: AttrState, data: Signal<series::Data<X, Y>>) -> Self {
        Self {
            attr,
            data: Data {
                x_range: create_memo(move |_| data.with(|data| data.x_range().cloned())),
                y_range: create_memo(move |_| data.with(|data| data.y_range().cloned())),
            },
        }
    }
}

impl<X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static> State<X, Y> {
    pub fn new(
        pre: PreState<X, Y>,
        node: &UseWatchedNode,
        layout: Layout,
        proj: Signal<Projection>,
        data: Signal<series::Data<X, Y>>,
    ) -> Self {
        let mouse_chart = node.mouse_chart;
        let mouse_hover_inner = node.mouse_hover_inner(layout.inner);

        // Data
        let mouse_hover_data = create_memo(move |_| {
            let (mouse_x, mouse_y) = mouse_chart.get();
            proj.get().svg_to_data(mouse_x, mouse_y)
        });
        let mouse_hover_nearest_data_x = create_memo(move |_| {
            let (data_x, _) = mouse_hover_data.get();
            data.with(|data| data.nearest_x_position(data_x))
        });
        let mouse_hover_nearest_svg_x = create_memo(move |_| {
            let (data_x, _) = mouse_hover_data.get();
            let (svg_x, _) = proj.get().data_to_svg(data_x, 0.0);
            svg_x
        });

        Self {
            attr: pre.attr,
            data: pre.data,
            layout,
            projection: proj,
            svg_zero: create_memo(move |_| proj.get().data_to_svg(0.0, 0.0)),
            page_bounds: node.bounds,
            mouse_page: node.mouse_page,
            mouse_chart,
            mouse_hover_chart: node.mouse_chart_hover,
            mouse_hover_inner,

            mouse_hover_data,
            mouse_hover_nearest_data_x,
            mouse_hover_nearest_svg_x,
        }
    }
}
