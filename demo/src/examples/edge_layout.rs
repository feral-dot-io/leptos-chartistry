use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // Add names to our lines for the legend to use
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_name("sunshine"))
        .line(Line::new(|data: &MyData| data.y2).with_name("rain"));
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            top=RotatedLabel::middle("Your chart title")
            left=TickLabels::aligned_floats()
            bottom=vec![
                TickLabels::aligned_floats().into_edge(),
                Legend::end().into_edge()
            ]
        />
    }
}
