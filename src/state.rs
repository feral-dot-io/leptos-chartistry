use crate::{
    bounds::Bounds, layout::Layout, line::UseLine, projection::Projection, series::Data,
    use_watched_node::UseWatchedNode, Font, Padding, UseSeries,
};
use leptos::signal_prelude::*;

#[derive(Clone, Debug)]
pub struct PreState<X: 'static, Y: 'static> {
    pub debug: Signal<bool>,
    pub font: Signal<Font>,
    pub padding: Signal<Padding>,
    // Data
    data: Signal<Data<X, Y>>,
    pub lines: Memo<Vec<UseLine>>,
    pub x_range: Memo<Option<(X, X)>>,
    pub y_range: Memo<Option<(Y, Y)>>,
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

    /// X position of nearest data from mouse data
    pub nearest_pos_x: Memo<f64>,
    /// X coord of nearest mouse data in SVG space
    pub nearest_svg_x: Memo<f64>,
    /// X value of nearest mouse data
    pub nearest_data_x: Memo<Option<X>>,
    /// Y values of nearest mouse data. Index corresponds to line index.
    pub nearest_data_y: Memo<Vec<(UseLine, Option<Y>)>>,
}

impl<X: Clone + PartialEq + 'static, Y: Clone + PartialEq + 'static> PreState<X, Y> {
    pub fn new(
        debug: Signal<bool>,
        font: Signal<Font>,
        padding: Signal<Padding>,
        series: UseSeries<X, Y>,
    ) -> Self {
        let UseSeries { lines, data } = series;

        let lines = create_memo(move |_| {
            let mut lines = lines.clone();
            lines.sort_by_key(|line| line.name.get());
            lines
        });

        Self {
            debug,
            font,
            padding,

            data,
            lines,
            x_range: create_memo(move |_| data.with(|data| data.x_range().cloned())),
            y_range: create_memo(move |_| data.with(|data| data.y_range().cloned())),
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
        let data = pre.data;
        // Mouse
        let mouse_chart = node.mouse_chart;
        let hover_inner = node.mouse_hover_inner(layout.inner);

        // Data
        let hover_data = create_memo(move |_| {
            let (mouse_x, mouse_y) = mouse_chart.get();
            proj.get().svg_to_data(mouse_x, mouse_y)
        });
        let nearest_pos_x = create_memo(move |_| {
            let (pos_x, _) = hover_data.get();
            data.with(|data| data.nearest_x_position(pos_x))
        });
        let nearest_svg_x = create_memo(move |_| {
            let data_x = nearest_pos_x.get();
            let (svg_x, _) = proj.get().data_to_svg(data_x, 0.0);
            svg_x
        });
        let nearest_data_x = create_memo(move |_| {
            let (pos_x, _) = hover_data.get();
            data.with(|data| data.nearest_x(pos_x).cloned())
        });
        let nearest_data_y = create_memo(move |_| {
            let pos_x = nearest_pos_x.get();
            data.with(|data| {
                pre.lines
                    .get()
                    .into_iter()
                    .map(|line| {
                        let y_value = data.nearest_y(pos_x, line.id);
                        (line, y_value)
                    })
                    .collect::<Vec<_>>()
            })
        });

        Self {
            pre,
            layout,
            projection: proj,
            svg_zero: create_memo(move |_| proj.get().data_to_svg(0.0, 0.0)),

            page_bounds: node.bounds,
            mouse_page: node.mouse_page,
            mouse_chart,
            hover_chart: node.mouse_chart_hover,
            hover_inner,

            nearest_pos_x,
            nearest_svg_x,
            nearest_data_x,
            nearest_data_y,
        }
    }
}
