use chrono::prelude::*;
use leptos::*;
use leptos_chartistry::*;

const DEFAULT_FONT_HEIGHT: f64 = 16.0;
const DEFAULT_FONT_WIDTH: f64 = 10.0;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

pub struct Wave {
    x: f64,
    sine: Vec<f64>,
    cosine: Vec<f64>,
}

fn load_data() -> Vec<Wave> {
    use std::f64::consts::PI;
    let mut data = Vec::new();
    for i in 0..1000 {
        let x = i as f64 / 1000.0 * PI * 2.0 * 2.0;
        let (sine, cosine): (Vec<_>, Vec<_>) = (0..10)
            .map(|j| {
                let v = x + 2.0 * PI * j as f64 / 10.0;
                let sin = v.sin() * 0.5 + 0.5;
                let cos = (v.cos() + PI) * (PI / 100.0);
                (sin, cos)
            })
            .unzip();
        data.push(Wave { x, sine, cosine });
    }
    data
}

pub fn f64_to_dt(at: f64) -> DateTime<Utc> {
    let nsecs = (at.fract() * 1_000_000_000.0).round() as u32;
    Utc.timestamp_opt(at as i64, nsecs).unwrap()
}

#[component]
pub fn App() -> impl IntoView {
    let (debug, set_debug) = create_signal(false);
    let (percent, set_percent) = create_signal(false);

    // Font
    let (font_height, set_font_height) = create_signal(DEFAULT_FONT_HEIGHT);
    let (font_width, set_font_width) = create_signal(DEFAULT_FONT_WIDTH);
    let font = Signal::derive(move || Font::new(font_height.get(), font_width.get()));

    // Padding
    let (padding_value, set_padding) = create_signal(DEFAULT_FONT_WIDTH);
    let padding = Signal::derive(move || Padding::from(padding_value.get()));

    // Data
    let (data, _) = create_signal(load_data());
    let mut series = Series::new(|w: &Wave| f64_to_dt(w.x));

    // Sine lines
    let mut lines = Vec::new();
    for i in 0..10 {
        lines.push(
            Line::new(move |w: &Wave| {
                if percent.get() {
                    let total = w.sine.iter().sum::<f64>();
                    w.sine[i] / total
                } else {
                    w.sine[i]
                }
            })
            .set_name(format!("Sine{}", i))
            .set_width(2.0),
        );
    }
    series = series.lines(lines);
    series = series.line(|_: &_| f64::NAN);

    // Cosine stack
    series = series.stack((0..10).map(|i| {
        Line::new(move |w: &Wave| w.cosine[i])
            .set_name(format!("Cosine{}", i))
            .set_width(4.0)
    }));

    let (anchor, _) = create_signal(Anchor::Middle);
    let (text, _) = create_signal("Hello and welcome to Chartistry!".to_string());
    let top_label = RotatedLabel::new(anchor, text);
    let left_ticks = TickLabels::aligned_floats().set_min_chars(20);
    let bottom_ticks = TickLabels::timestamps();

    view! {
        <h1>"Chartistry"</h1>
        <form>
            <p>
                <label>
                    <input type="checkbox" checked=percent on:input=move |ev| set_percent.set(event_target_checked(&ev)) />
                    "Percent"
                </label>
            </p>
            <p>
                <label>
                    <input type="checkbox" checked=debug on:input=move |ev| set_debug.set(event_target_checked(&ev)) />
                    "Debug"
                </label>
            </p>
            <p>
                <label>
                    "Font height"
                    <input type="number" step="0.5" min="0.1" value=font_height on:input=move |ev| set_font_height.set(event_target_value(&ev).parse().unwrap_or(DEFAULT_FONT_HEIGHT)) />
                </label>
            </p>
            <p>
                <label>
                    "Font width"
                    <input type="number" step="0.5" min="0.1" value=font_width on:input=move |ev| set_font_width.set(event_target_value(&ev).parse().unwrap_or(DEFAULT_FONT_WIDTH)) />
                </label>
            </p>
            <p>
                <label>
                    "Padding"
                    <input type="number" step="0.5" min="0.1" value=padding_value on:input=move |ev| set_padding.set(event_target_value(&ev).parse().unwrap_or(DEFAULT_FONT_WIDTH)) />
                </label>
            </p>
        </form>

        <Chart
            aspect_ratio=AspectRatio::outer_width(1100.0, 0.6)
            font=font
            debug=debug
            padding=padding

            top=vec![top_label.to_horizontal(), Legend::end().to_horizontal()]
            right=vec![Legend::middle()]
            bottom=vec![&bottom_ticks]
            left=vec![&left_ticks]
            inner=vec![
                AxisMarker::left_edge(),
                AxisMarker::horizontal_zero(),
                GridLine::horizontal(&left_ticks),
                GridLine::vertical(&bottom_ticks),
                GuideLine::x_axis_over_data(),
                GuideLine::y_axis(),
            ]
            tooltip=Tooltip::left_cursor(bottom_ticks, left_ticks).sort_by_f64_descending()

            series=series
            min_y=Some(-1.0)
            max_y=Some(2.0)
            data=data
        />
    }
}
