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
        let sine = x.sin() * SCALE + 1.1;
        let cosine = x.cos() * SCALE + 1.1;
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
    let series = Series::new(&|w: &Wave| f64_to_dt(w.x))
        .set_x_range(None, f64_to_dt(15.0))
        .set_y_range(-1.0, None)
        .add(Line::new("Sphinx"), &|w: &Wave| w.sine)
        .add(Line::new("Cophine"), &|w: &Wave| w.cosine)
        .use_data::<Vec<_>>(data);

    let (anchor, _) = create_signal(Anchor::Middle);
    let (text, _) = create_signal("Hello and welcome to Chartistry!".to_string());
    let top_label = RotatedLabel::new(anchor, text);
    let snippet = Snippet::horizontal().set_padding(Padding::zero());
    let left_ticks = TickLabels::aligned_floats();
    let bottom_ticks = TickLabels::timestamps();

    let chart = Chart::new(font, series)
        .inherit_padding(padding)
        .inherit_debug(debug)
        // Labels
        .add_top(&top_label)
        .add_top(Legend::end(Snippet::horizontal()))
        // Ticks
        .add_left(&left_ticks)
        .add_bottom(&bottom_ticks)
        // Axis lines
        .add(AxisMarker::left_edge())
        .add(AxisMarker::horizontal_zero())
        // Grid lines
        .add(GridLine::horizontal(&left_ticks))
        .add(GridLine::vertical(&bottom_ticks))
        // Guide lines
        .add(GuideLine::x_axis())
        .add(GuideLine::y_axis())
        // Legend
        .add(InsetLegend::right(&snippet))
        // Tooltip
        .overlay(Tooltip::left_cursor(&snippet, &bottom_ticks, &left_ticks));

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
        <Chart chart=chart width=1100.0 height=600.0 />
    }
}
