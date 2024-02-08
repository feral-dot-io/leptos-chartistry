use super::{MyData, EXAMPLE_ASPECT_RATIO};
use leptos::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // A variety of ways to create new colours
    const BLACK: Colour = Colour::new(0, 0, 0);
    const THISTLE: Colour = Colour::new(216, 191, 216);
    let sea_green: Colour = "#20b2aa".parse().unwrap();
    const RED: Colour = Colour::new(255, 0, 0);
    const BLUE_VIOLET: Colour = Colour::new(0, 0, 255);

    // We can also describe a colour scheme for our Series:
    // For non-stacked, colours are picked one after the other and then repeat
    // For stacked lines, colours are interpolated between the first and last
    let scheme = ColourScheme::new(RED, vec![BLUE_VIOLET]);

    // Add names to our lines for the legend to use
    let series = Series::new(|data: &MyData| data.x)
        .line(
            Line::new(|data: &MyData| data.y1)
                .with_name("roses")
                // Manually specify the colour of a line
                .with_colour(RED),
        )
        .line(
            Line::new(|data: &MyData| data.y2)
                .with_name("violets")
                .with_colour(BLUE_VIOLET),
        )
        // Or specify the colour scheme (this gives the same as above but more flexible)
        .with_colours(scheme);

    view! {
        <Chart
            aspect_ratio=EXAMPLE_ASPECT_RATIO
            debug=debug
            series=series
            data=data

            inner=vec![
                // Most drawn elements can have their colour changed
                AxisMarker::left_edge().with_colour(BLACK).into_inner(),
                YGridLine::default().with_colour(THISTLE).into_inner(),
                YGuideLine::over_mouse().with_colour(sea_green).into_inner(),
                // Legends pick their colours from the lines they're describing
                InsetLegend::bottom_right().into_inner(),
            ]
        />
    }
}
