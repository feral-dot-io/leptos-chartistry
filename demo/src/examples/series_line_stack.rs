use super::MyData;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
    // Put our lines into a stack
    let stack = Stack::new()
        .line(Line::new(|data: &MyData| data.y1).with_name("fairies"))
        .line(Line::new(|data: &MyData| data.y2).with_name("pixies"));

    let series = Series::new(|data: &MyData| data.x)
        .stack(stack)
        // Start from zero
        .with_min_y(0.0);
    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            // Decorate our chart
            top=RotatedLabel::middle("Bottom of the garden")
            left=TickLabels::aligned_floats()
            bottom=Legend::end()
            inner=[
                // Standard set of inner layout options
                AxisMarker::left_edge().into_inner(),
                AxisMarker::bottom_edge().into_inner(),
                XGridLine::default().into_inner(),
                YGridLine::default().into_inner(),
                YGuideLine::over_mouse().into_inner(),
                XGuideLine::over_data().into_inner(),
            ]
            tooltip=Tooltip::left_cursor().show_x_ticks(false)
        />
    }
}
