use super::{MyData, EXAMPLE_ASPECT_RATIO};
use leptos::*;
use leptos_chartistry::*;

#[component]
pub fn Example(data: Signal<Vec<MyData>>) -> impl IntoView {
    // The names of our line series will show up on our legend
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_name("asdf"))
        .line(Line::new(|data: &MyData| data.y2).with_name("qwer"))
        .with_x_range(0.0, 10.0)
        .with_y_range(0.0, 10.0);
    view! {
        <Chart
            aspect_ratio=EXAMPLE_ASPECT_RATIO
            series=series
            data=data
            // Place legends on all edges for the sake of demonstration
            top=Legend::middle()
            bottom=Legend::end()
            left=Legend::start()
            right=Legend::end()
        />
    }
}
