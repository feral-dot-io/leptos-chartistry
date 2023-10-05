use leptos::*;
use leptos_chartistry::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let (em, _) = create_signal(16.0);
    let (ex, _) = create_signal(10.0);
    let (debug, _) = create_signal(false);

    let (width, _) = create_signal(1100.0);
    let font = Signal::derive(move || Font::new(em.get(), ex.get()));
    let padding = Signal::derive(move || Padding::from(ex.get()));

    let chart = Chart::new(width, 600.0, font)
        .with_padding(padding)
        .with_debug(debug)
        .add_top(RotatedLabel::middle("Hello and welcome to chartistry!"))
        .add_bottom(RotatedLabel::middle("Hello and welcome to chartistry!"))
        .add_left(RotatedLabel::middle("Hello and welcome to chartistry!"))
        .add_right(RotatedLabel::middle("Hello and welcome to chartistry!"));

    view! {
        <Chart chart=chart />
    }
}
