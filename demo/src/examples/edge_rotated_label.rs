use super::{MyData, EXAMPLE_ASPECT_RATIO};
use leptos::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .line(|data: &MyData| data.y1)
        .line(|data: &MyData| data.y2)
        .with_x_range(0.0, 10.0)
        .with_y_range(0.0, 10.0);
    view! {
        <Chart
            aspect_ratio=EXAMPLE_ASPECT_RATIO
            debug=debug
            series=series
            data=data

            top=RotatedLabel::middle("Chart title")
            left=RotatedLabel::start("Side edges are rotated...")
            right=RotatedLabel::end("...to point inwards.")
        />
    }
}
