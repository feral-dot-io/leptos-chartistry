use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

// Move Y so that values cross zero to demonstrate a diverging gradient
const Y_OFFSET: f64 = -6.0;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .line(
            Line::new(|data: &MyData| data.y1 + Y_OFFSET)
                .with_width(5.0)
                // Add a linear gradient that changes the line colour based on
                // the Y value.
                .with_gradient(LINEAR_GRADIENT),
        )
        .line(
            Line::new(|data: &MyData| data.y2 + Y_OFFSET)
                .with_width(5.0)
                // Add a diverging gradient that also changes the line colour
                // based on the Y value except it has a central value where the
                // gradient flips. In our case we show blue below zero and red
                // above.
                .with_gradient(DIVERGING_GRADIENT),
        );
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data
            // Show the zero line -- where a diverging gradient would flip
            inner=AxisMarker::horizontal_zero()
        />
    }
}
