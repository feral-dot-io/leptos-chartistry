use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

const WHITE: Colour = Colour::from_rgb(255, 255, 255);

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .line(
            Line::new(|data: &MyData| data.y1)
                .with_name("laughter")
                // Add a marker to each point on the line
                .with_marker(MarkerShape::Circle),
        )
        .line(
            Line::new(|data: &MyData| data.y2)
                .with_name("tears")
                .with_marker(
                    // We can also decorate our markers. Here we put a border on
                    // the marker that's the same as the line and then set the
                    // marker colour to white. This gives a hollow marker.
                    Marker::from_shape(MarkerShape::Circle)
                        .with_colour(WHITE)
                        // Note: default border colour is the line colour
                        .with_border_width(1.0),
                ),
        );

    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            // Markers are also shown in the legend
            bottom=Legend::end()
        />
    }
}
