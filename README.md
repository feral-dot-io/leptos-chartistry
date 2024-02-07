# Leptos Chartistry

<p>
  <a href="https://crates.io/crates/leptos-chartistry">
    <img src="https://img.shields.io/crates/v/leptos-chartistry.svg?style=for-the-badge" alt="Crates.io version" />
  </a>
  <a href="https://docs.rs/leptos-chartistry">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=for-the-badge" alt="Docs.rs" />
  </a>
</p>

Leptos Chartistry is an extensible charting library for [Leptos](https://github.com/leptos-rs/leptos). It provides a simple and easy to use `<Chart>` component.

- [Interactive demo](https://feral-dot-io.github.io/leptos-chartistry/)
- [Usage examples](https://feral-dot-io.github.io/leptos-chartistry/examples)
- [API documentation](https://docs.rs/leptos-chartistry)
- If you like or use Chartistry, consider giving it a star.

## Getting started

To add Leptos Chartistry to your project, run the following command:

```bash
cargo add leptos-chartistry
```

Next, use `<Chart ... />` to render a chart in your Leptos application. Here's a small example:

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

- The first priority is getting rid of bugs, issues and glaring omissions -- please report them!
- I am looking for feedback on the public API to avoid future churn.
- Following this I'd like to drive forward with examples and usecases. If you have a usecase that you think would be a good fit for Chartistry, please let me know with examples found on the web.
