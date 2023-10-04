use leptos::*;
use leptos_chartistry::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let font = Font::new(16.0, 10.0);
    let chart =
        Chart::new(font).add_text_label(RotatedLabel::middle("Hello and welcome to chartistry!"));

    view! {
        <Chart chart=chart />
    }
}
