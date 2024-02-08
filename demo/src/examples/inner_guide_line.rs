use super::{MyData, EXAMPLE_ASPECT_RATIO};
use leptos::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .line(|data: &MyData| data.y1)
        .line(|data: &MyData| data.y2);
    view! {
        <Chart
            aspect_ratio=EXAMPLE_ASPECT_RATIO
            debug=debug
            series=series
            data=data

            inner=vec![
                // Draw a line on the chart area when the mouse hovers
                XGuideLine::over_mouse().into_inner(),
                // Instead, they be drawn over the nearest data point
                // This creates a "snap-to" effect
                YGuideLine::over_data().into_inner()
            ]
        />
    }
}
