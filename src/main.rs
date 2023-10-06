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

#[component]
pub fn App() -> impl IntoView {
    let (em, _) = create_signal(16.0);
    let (ex, _) = create_signal(10.0);
    let (debug, _) = create_signal(false);

    let (width, _) = create_signal(1100.0);
    let font = Signal::derive(move || Font::new(em.get(), ex.get()));
    let padding = Signal::derive(move || Padding::from(ex.get()));

    let (data, _) = create_signal(load_data());
    let series = Series::new(&|w: &Wave| w.x)
        .add(Line::new("Sphinx"), &|w: &Wave| w.sine)
        .add(Line::new("Cophine"), &|w: &Wave| w.cosine)
        .with_data::<Vec<_>>(data);

    let (anchor, _) = create_signal(Anchor::Middle);
    let (text, _) = create_signal("Hello and welcome to Chartistry!".to_string());
    let chart = Chart::new(width, 600.0, font, series)
        .with_padding(padding)
        .with_debug(debug)
        .add_top(RotatedLabel::new(anchor, text))
        .add_bottom(RotatedLabel::new(anchor, text))
        .add_left(RotatedLabel::new(anchor, text))
        .add_right(RotatedLabel::new(anchor, text));

    view! {
        <Chart chart=chart />
    }
}
