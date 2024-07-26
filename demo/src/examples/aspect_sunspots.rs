use crate::data;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn AspectRatioSunspots(debug: Signal<bool>) -> impl IntoView {
    let x_ticks = TickLabels::from_generator(Timestamps::from_period(Period::Year));
    let y_ticks = TickLabels::aligned_floats();

    // Our sunspot data from https://www.sidc.be/silso/
    let series =
        Series::new(|data: &data::Sunspots| data.year).line(|data: &data::Sunspots| data.sunspots);

    // Width slider
    let (width, set_width) = create_signal(0.8);
    let SLIDER_RANGE: f64 = 60.0;
    let frame_width = move || format!("{}%", (100.0 - SLIDER_RANGE) + SLIDER_RANGE * width.get());
    let change_width = move |ev| {
        let value = event_target_value(&ev)
            .parse::<f64>()
            .unwrap_or_default()
            .clamp(0.0, 1.0);
        set_width.set(value);
    };

    view! {
        <p>"The following chart is of sunspot activity from 1700 to 2020. Its "
        "width stretches the page and has a fixed height. Note the large ups "
        "and downs in the chart whose interpretation changes with page width."</p>
        <div style:width=frame_width>
            <Chart
                debug=debug
                aspect_ratio=AspectRatio::from_env_width(400.0)

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
        </div>

        <p>"Try changing the chart width and see how the chart looks different "
        "rather than losing detail:"</p>
        <p><input style="width: 100%"
            type="range" min="0" max="1" step="0.01"
            value=width on:input=change_width /></p>

        <p>"We can make more sense of the data—and see more useful patterns—by "
        "setting a fixed ratio for a chart. This can be done by supplying two "
        "variables from the formula: " <code>"width / height = ratio"</code>.</p>
        <div style:width=frame_width style="height: 90px;">
            <Chart
                debug=debug
                aspect_ratio=AspectRatio::from_env_width_apply_ratio(15.0)

                bottom=x_ticks.clone()
                inner=vec![
                    XGridLine::from_ticks(x_ticks.clone()).into_inner(),
                    XGuideLine::over_data().into_inner(),
                ]
                tooltip=Tooltip::left_cursor()

                series=series
                data=data::daily_sunspots />
        </div>

        <p>"Source: "<a href="https://www.sidc.be/silso/">"Sunspot data from "
        "the World Data Center SILSO"</a>", Royal Observatory of Belgium, "
        "Brussels"</p>
    }
}
