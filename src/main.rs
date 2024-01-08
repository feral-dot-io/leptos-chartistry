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
    sine: f64,
    cosine: f64,
}

fn load_data() -> Vec<Wave> {
    const SCALE: f64 = 1.0;
    let mut data = Vec::new();
    for i in 0..1000 {
        let x = i as f64 / 1000.0 * std::f64::consts::PI * 2.0 * 2.0;
        let sine = x.sin() * SCALE + 1.0;
        let cosine = x.cos() * SCALE + 1.0;
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

    // Font
    let (font_height, set_font_height) = create_signal(DEFAULT_FONT_HEIGHT);
    let (font_width, set_font_width) = create_signal(DEFAULT_FONT_WIDTH);
    let font = Signal::derive(move || Font::new(font_height.get(), font_width.get()));

    // Padding
    let (padding_value, set_padding) = create_signal(DEFAULT_FONT_WIDTH);
    let padding = Signal::derive(move || Padding::from(padding_value.get()));

    // Data
    let (data, _) = create_signal(load_data());
    let series = Series::new(|w: &Wave| f64_to_dt(w.x))
        .add_series(Line::new(&|w: &Wave| w.sine).set_name("A").set_width(5.0))
        .add_series(Line::new(&|w: &Wave| w.cosine).set_name("B").set_width(5.0))
        //.add_series(Line::new(&|_: &Wave| f64::NAN))
        .add_series(Stack::new(vec![
            Line::new(&|w: &Wave| w.sine).set_name("Stack-A"),
            Line::new(&|w: &Wave| w.cosine).set_name("Stack-B"),
            //Line::new(&|_: &Wave| f64::NAN),
        ]));

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

            //top=vec![top_label.to_horizontal(), Legend::end().to_horizontal()]
            top=HorizontalVec::default().push(top_label).push(Legend::end())
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
                InsetLegend::top_right()
            ]
            tooltip=Tooltip::left_cursor(bottom_ticks, left_ticks).sort_by_f64_descending()

            series=series
            data=data
        />
    }
}
