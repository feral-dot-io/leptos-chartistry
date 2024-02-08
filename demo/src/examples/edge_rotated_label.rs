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

            left=RotatedLabel::start("Side edges...")
            right=RotatedLabel::end("...point inwards.")
            bottom=vec![
                RotatedLabel::middle("You can also add...").into_edge(),
                RotatedLabel::middle("...multiple labels.").into_edge(),
            ]
        />
    }
}
