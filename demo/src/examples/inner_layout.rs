use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // Add names to our lines for the legend to use
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_name("tea"))
        .line(Line::new(|data: &MyData| data.y2).with_name("coffee"));
    let x_ticks = TickLabels::default();
    let y_ticks = TickLabels::default();
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            inner=vec![
                // You'll probably always want axis markers
                AxisMarker::left_edge().into_inner(),
                // If meaningful for your data, mark zero instead of the axis
                AxisMarker::horizontal_zero().into_inner(),
                // Align grid lines to the tick labels you show
                XGridLine::from_ticks(x_ticks).into_inner(),
                YGridLine::from_ticks(y_ticks).into_inner(),
                // Add interactivity to help read your chart
                YGuideLine::over_mouse().into_inner(),
                // Your reader might be trying to read a data point, not the
                // axis value. Use `over_data` to help with this
                XGuideLine::over_data().into_inner(),
                // Be careful with inset legends, as they can overlap your data
                InsetLegend::bottom_right().into_inner(),
            ]
        />
    }
}
