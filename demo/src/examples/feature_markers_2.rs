use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

const BACKGROUND: Colour = Colour::from_rgb(255, 255, 255);
const BORDER_WIDTH: f64 = 2.0;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .line(
            Line::new(|data: &MyData| data.y1)
                .with_name("lions")
                .with_marker(
                    Marker::from_shape(MarkerShape::Cross)
                        // Creating a border around the marker creates a small gap
                        .with_border(BACKGROUND)
                        .with_border_width(BORDER_WIDTH),
                ),
        )
        .line(
            Line::new(|data: &MyData| data.y2)
                .with_name("tigers")
                .with_marker(
                    Marker::from_shape(MarkerShape::Plus)
                        .with_border(BACKGROUND)
                        .with_border_width(BORDER_WIDTH),
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
