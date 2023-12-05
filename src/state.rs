use crate::{
    bounds::Bounds, layout::Layout, projection::Projection, series::UseLine,
    use_watched_node::UseWatchedNode, Font, Padding, UseData,
};
use leptos::signal_prelude::*;

#[derive(Clone, Debug)]
pub struct PreState<X: 'static, Y: 'static> {
    pub debug: Signal<bool>,
    pub font: Signal<Font>,
    pub padding: Signal<Padding>,
    pub data: UseData<X, Y>,
}

#[derive(Clone, Debug)]
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

    /// X coord of nearest mouse data in SVG space
    pub nearest_svg_x: Memo<f64>,
    /// X value of nearest mouse data
    pub nearest_data_x: Memo<Option<X>>,
    /// Y values of nearest mouse data. Index corresponds to line index.
    pub nearest_data_y: Memo<Vec<(UseLine, Option<Y>)>>,
}

impl<X, Y> PreState<X, Y> {
    pub fn new(
        debug: Signal<bool>,
        font: Signal<Font>,
        padding: Signal<Padding>,
        data: UseData<X, Y>,
    ) -> Self {
        Self {
            debug,
            font,
            padding,
            data,
        }
    }
}

impl<X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static> State<X, Y> {
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
        let hover_data = create_memo(move |_| {
            let (mouse_x, mouse_y) = mouse_chart.get();
            proj.get().svg_to_position(mouse_x, mouse_y)
        });
        let hover_data_x = Signal::derive(move || hover_data.get().0);

        let nearest_pos_x = pre.data.nearest_position_x(hover_data_x);
        let nearest_svg_x = create_memo(move |_| {
            let data_x = nearest_pos_x.get();
            let (svg_x, _) = proj.get().position_to_svg(data_x, 0.0);
            svg_x
        });

        let nearest_data_x = pre.data.nearest_data_x(hover_data_x);
        let nearest_data_y = {
            let series = pre.data.series;
            let values = pre.data.nearest_data_y(hover_data_x);
            create_memo(move |_| {
                series
                    .get()
                    .into_iter()
                    .zip(values.iter())
                    .map(|(series, value)| (series, value.get()))
                    .collect::<Vec<_>>()
            })
        };

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

            nearest_svg_x,
            nearest_data_x,
            nearest_data_y,
        }
    }
}
