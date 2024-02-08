use super::{MyData, EXAMPLE_ASPECT_RATIO};
use leptos::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // Add names to our lines for the legend to use
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_name("pears"))
        .line(Line::new(|data: &MyData| data.y2).with_name("apples"));
    view! {
        <Chart
            aspect_ratio=EXAMPLE_ASPECT_RATIO
            debug=debug
            series=series
            data=data

            // Add a tooltip to the chart
            tooltip=Tooltip::left_cursor()
        />
    }
}
