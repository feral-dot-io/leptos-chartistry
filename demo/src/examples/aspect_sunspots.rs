use crate::data;
use leptos::*;
use leptos_chartistry::*;

#[component]
pub fn AspectRatioSunspots(debug: Signal<bool>) -> impl IntoView {
    let x_ticks = TickLabels::from_generator(Timestamps::from_period(Period::Year));
    let y_ticks = TickLabels::aligned_floats();

    let series =
        Series::new(|data: &data::Sunspots| data.year).line(|data: &data::Sunspots| data.sunspots);
    view! {
        <p>"The following chart is of sunspot activity from 1700 to 2020. Its "
        "width stretches the page and has a fixed height. Note the large ups "
        "and downs in the chart whose interpretation changes with page width."</p>
        <Chart
            debug=debug
            aspect_ratio=AspectRatio::inner_ratio(600.0, 600.0)

            top=RotatedLabel::middle("Daily sunspots")
            left=y_ticks.clone()
            bottom=x_ticks.clone()
            inner=vec![
                AxisMarker::left_edge().into_inner(),
                AxisMarker::horizontal_zero().into_inner(),
                XGridLine::from_ticks(x_ticks.clone()).into_inner(),
                YGridLine::from_ticks(y_ticks.clone()).into_inner(),
                XGuideLine::over_data().into_inner(),
                YGuideLine::over_mouse().into_inner(),
            ]
            tooltip=Tooltip::left_cursor()

            series=series.clone()
            data=data::daily_sunspots />

        <p>"We can make more sense of the data—and see more useful patterns—by "
        "setting a fixed ratio for a chart. This can be done by supplying two "
        "variables from the formula: " <code>"width / height = ratio"</code></p>
        <Chart
            debug=debug
            aspect_ratio=AspectRatio::inner_width(30.0, 25.0)

            bottom=x_ticks.clone()
            inner=vec![
                XGridLine::from_ticks(x_ticks.clone()).into_inner(),
                XGuideLine::over_data().into_inner(),
            ]
            tooltip=Tooltip::left_cursor()

            series=series
            data=data::daily_sunspots />

        <p>"Source: "<a href="https://www.sidc.be/silso/">"Sunspot data from "
        "the World Data Center SILSO"</a>", Royal Observatory of Belgium, "
        "Brussels"</p>
    }
}
