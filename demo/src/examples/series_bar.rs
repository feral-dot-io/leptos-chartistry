use super::MyData;
use leptos::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // Lines are added to the series
    let series = Series::new(|data: &MyData| data.x)
        .bar(Bar::new(|data: &MyData| data.y1).with_name("trees"))
        .bar(Bar::new(|data: &MyData| data.y2).with_name("plants"));
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            // Decorate our chart
            top=RotatedLabel::middle("My garden")
            left=TickLabels::aligned_floats()
            bottom=Legend::end()
            inner=[
                AxisMarker::left_edge().into_inner(),
                AxisMarker::bottom_edge().into_inner(),
                YGuideLine::over_mouse().into_inner(),
                XGuideLine::over_data().into_inner(),
            ]
            tooltip=Tooltip::left_cursor().show_x_ticks(false)
        />
    }
}
