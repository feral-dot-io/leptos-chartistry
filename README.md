# Leptos Chartistry

An extensible charting library for [Leptos](https://github.com/leptos-rs/leptos).

- [Interactive demo](#todo)
- [Usage examples](#todo)
- [API documentation](#todo)
- If you like or use Chartistry, consider giving it a star.

## Getting started

Run `cargo add leptos-chartistry` to add the library to your project. Next, use `<Chart ... />` to render a chart.

```rust
use leptos::*;
use leptos_chartistry::*;

#[derive(Clone, PartialEq)]
pub struct MyData {
    pub x: f64,
    pub first: f64,
    pub second: f64,
}

#[component]
pub fn MyFirstChart(data: Signal<Vec<MyData>>) -> impl IntoView {
    // A chart with two lines
    let series = Series::new(|d: &MyData| d.x)
        .line(Line::new(|d: &MyData| d.first).with_name("First"))
        .line(Line::new(|d: &MyData| d.second).with_name("Second"));

    view! {
        <Chart
            // Size of the chart (not including layout options)
            aspect_ratio=AspectRatio::inner_ratio(800.0, 600.0)
            // Layout options
            top=RotatedLabel::middle("My first chart")
            left=TickLabels::aligned_floats()
            inner=vec![
                AxisMarker::left_edge().into_inner(),
                AxisMarker::horizontal_zero().into_inner(),
                InsetLegend::top_right().into_inner(),
            ]
            tooltip=Tooltip::left_cursor().with_sort_by(TooltipSortBy::Descending)

            series=series
            data=data
        />
    }
}
```

## Feedback

This project is in its early stages and feedback is welcome. Here are some things we're particularly interested in:

- I am looking for feedback on the public API.
- Usecases and examples of features from other charts found on the web whose features could be incorporated.

## Licence

MPL2
