use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // Add names to our lines for the legend to use
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_name("pears"))
        .line(Line::new(|data: &MyData| data.y2).with_name("apples"));
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data
            // Show a legend, left-to-right, placed in the middle
            top=Legend::start()
            // Vertical legends are a top-to-bottom list, scrollable on overflow
            right=Legend::middle()
            bottom=Legend::end()
        />
    }
}
