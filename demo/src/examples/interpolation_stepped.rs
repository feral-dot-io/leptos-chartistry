use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // Add names to our lines for the legend to use
    let series = Series::new(|data: &MyData| data.x)
        .line(
            Line::new(|data: &MyData| data.y1)
                .with_name("horizontal")
                .with_marker(MarkerShape::Square)
                // Horizontal steps move along the x-axis first, then the y-axis.
                // You can also use `Step::Vertical` to move along the y-axis first.
                .with_interpolation(Step::Horizontal),
        )
        .line(
            Line::new(|data: &MyData| data.y2)
                .with_name("middle")
                .with_marker(MarkerShape::Diamond)
                // Alternatively you can move across half the x-axis first, then
                // all of the y-axis, then the second half of the x-axis. This
                // creates a step in the middle of each point.
                .with_interpolation(Step::HorizontalMiddle),
        );
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data
            bottom=Legend::end()
        />
    }
}
