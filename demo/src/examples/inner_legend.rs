use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // Add names to our lines for the legend to use
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_name("cats"))
        .line(Line::new(|data: &MyData| data.y2).with_name("dogs"));
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            inner=vec![
                // Pick a place for your legend...
                InsetLegend::top().into_inner(),
                // ...and it will appear inside the chart area
                InsetLegend::bottom_right().into_inner()
            ]
        />
    }
}
