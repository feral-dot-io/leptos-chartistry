use super::MyData;
use leptos::*;
use leptos_chartistry::*;

const BACKGROUND: Colour = Colour::new(255, 255, 255);

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .line(
            Line::new(|data: &MyData| data.y1)
                .with_name("lions")
                .with_marker(
                    Marker::new(MarkerShape::Cross)
                        .with_border(BACKGROUND)
                        .with_border_width(3.0),
                ),
        )
        .line(
            Line::new(|data: &MyData| data.y2)
                .with_name("tigers")
                .with_marker(
                    Marker::new(MarkerShape::Plus)
                        .with_border(BACKGROUND)
                        .with_border_width(3.0),
                ),
        );

    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            bottom=Legend::start()
        />
    }
}
