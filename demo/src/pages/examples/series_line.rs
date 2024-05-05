use crate::examples::{load_data, series_line};
use leptos::*;

#[component]
pub fn Example() -> impl IntoView {
    let debug = create_rw_signal(false);
    let data = load_data();
    view! {
        <article id="examples">
            <series_line::Example debug=debug.into() data=data />
        </article>
    }
}
