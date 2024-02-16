use super::MyData;
use leptos::*;
use leptos_chartistry::*;
use leptos_meta::Style;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_name("sunlight"))
        .line(Line::new(|data: &MyData| data.y2).with_name("moonlight"));
    view! {
        // All elements drawn are given a class with the _chartistry_ prefix
        // which we can use to apply themes to our chart.
        <Style>"
            .my-theme {
                background-color: #333;

                /* Use 'fill' for filling text colour */
                fill: #ddd;

                /* Some elements (e.g., legend and tooltips) use HTML so we
                    still still need to set 'color' */
                color: #ddd;
            }

            /* We can set stroke (and fill) directly too */
            .my-theme ._chartistry_grid_line_x {
                stroke: #505050;
            }

            /* The tooltip uses inline CSS styles and so must be overridden */
            .my-theme ._chartistry_tooltip {
                border: 1px solid #fff !important;
                background-color: #333 !important;
            }
        "</Style>

        <div class="my-theme">
            <Chart
                aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
                debug=debug
                series=series
                data=data

                // Decorate our chart
                top=RotatedLabel::middle("Applying a theme")
                left=TickLabels::aligned_floats()
                bottom=Legend::start()
                inner=[
                    XGridLine::default().into_inner(),
                    // We can also use the `with_colour` method on some elements
                    YGridLine::default().with_colour("#505050".parse::<Colour>().unwrap()).into_inner(),
                    AxisMarker::left_edge().into_inner(),
                    AxisMarker::bottom_edge().into_inner(),
                    YGuideLine::over_mouse().into_inner(),
                    XGuideLine::over_data().into_inner(),
                ]
                tooltip=Tooltip::left_cursor().show_x_ticks(false)
            />
        </div>
    }
}
