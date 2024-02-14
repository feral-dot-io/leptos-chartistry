use super::MyData;
use leptos::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_gradient(LINEAR_GRADIENT))
        .line(Line::new(|data: &MyData| data.y2).with_gradient(LINEAR_GRADIENT));
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_width(250.0, 1.2)
            debug=debug
            series=series
            data=data
        />
    }
}
