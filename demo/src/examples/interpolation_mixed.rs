use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // Add names to our lines for the legend to use
    let series = Series::new(|data: &MyData| data.x)
        // Draw our two usual lines and use `Interpolation::Monotone` (the default).
        .line(Line::new(|data: &MyData| data.y1).with_interpolation(Interpolation::Monotone))
        .line(Line::new(|data: &MyData| data.y2).with_interpolation(Interpolation::Monotone))
        // Draw two more lines with the same data but this time using
        // `Interpolation::Linear` to see the difference particularly around
        // curves.
        .line(
            Line::new(|data: &MyData| data.y1)
                .with_colour(GUIDE_LINE_COLOUR)
                .with_marker(Marker::from_shape(MarkerShape::Circle).with_colour(GUIDE_LINE_COLOUR))
                .with_name("linear")
                .with_interpolation(Interpolation::Linear),
        )
        .line(
            Line::new(|data: &MyData| data.y2)
                .with_colour(GUIDE_LINE_COLOUR)
                .with_marker(Marker::from_shape(MarkerShape::Circle).with_colour(GUIDE_LINE_COLOUR))
                .with_interpolation(Interpolation::Linear),
        );
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data
        />
    }
}
