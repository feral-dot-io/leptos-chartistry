use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .line(|data: &MyData| data.y1)
        .line(|data: &MyData| data.y2);
    let y_ticks = TickLabels::aligned_floats();
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            inner=vec![
                // Draw grid lines aligned to the default tick labels
                XGridLine::default().into_inner(),
                // We can also specify our own tick labels for consistency e.g.,
                // we could have ticks aligned to only years rather than a
                // selection of periods -- we want our grid lines to follow
                YGridLine::from_ticks(y_ticks).into_inner()
            ]
        />
    }
}
