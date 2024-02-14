use super::MyData;
use leptos::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let gradient: ColourScheme = LINEAR_GRADIENT.into();
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_gradient(LINEAR_GRADIENT))
        .line(Line::new(|data: &MyData| data.y2).with_gradient(gradient.invert()));
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data
        />
    }
}
