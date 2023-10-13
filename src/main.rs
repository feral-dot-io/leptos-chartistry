use chrono::prelude::*;
use leptos::*;
use leptos_chartistry::*;

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
        let sine = x.sin() * SCALE;
        let cosine = x.cos() * SCALE;
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
    let (em, _) = create_signal(16.0);
    let (ex, _) = create_signal(10.0);
    let (debug, set_debug) = create_signal(false);

    let (width, _) = create_signal(1100.0);
    let font = Signal::derive(move || Font::new(em.get(), ex.get()));
    let padding = Signal::derive(move || Padding::from(ex.get()));

    let (data, _) = create_signal(load_data());
    let series = Series::new(&|w: &Wave| f64_to_dt(w.x))
        .add(Line::new("Sphinx"), &|w: &Wave| w.sine)
        .add(Line::new("Cophine"), &|w: &Wave| w.cosine)
        .use_data::<Vec<_>>(data);

    let (anchor, _) = create_signal(Anchor::Middle);
    let (text, _) = create_signal("Hello and welcome to Chartistry!".to_string());
    let top_label = RotatedLabel::new(anchor, text);

    let chart = Chart::new(width, 600.0, font, series)
        .inherit_padding(padding)
        .inherit_debug(debug)
        // Text labels
        .add_top(&top_label)
        .add_right(RotatedLabel::new(anchor, text))
        // Ticks
        .add_left(TickLabels::aligned_floats())
        .add_bottom(TickLabels::timestamps())
        // Legend
        .add_top(Legend::end(Snippet::horizontal()))
        // Axis lines
        .add(AxisMarker::bottom_edge())
        .add(AxisMarker::left_edge())
        .add(AxisMarker::horizontal_zero())
        // Grid lines
        .add(GridLine::horizontal(TickLabels::aligned_floats()))
        .add(GridLine::vertical(TickLabels::timestamps()));

    view! {
        <form>
            <p>
                <label>
                    <input
                        type="checkbox"
                        checked={debug}
                        on:input=move |ev| set_debug.set(event_target_checked(&ev)) />
                    "Debug"
                </label>
            </p>
        </form>

        <Chart chart=chart />
    }
}
