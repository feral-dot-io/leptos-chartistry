use crate::examples::{load_data, series_line};
use leptos::*;

#[component]
pub fn Example() -> impl IntoView {
    let debug = create_rw_signal(false);
    let data = load_data();
    let code = include_str!("../../examples/series_line.rs");
    view! {
        <article id="example-line-chart" class="example">
            <div class="background-box demo">
                <h1 class="connect-heading">"Line chart"</h1>
                <series_line::Example debug=debug.into() data=data />
            </div>
            <div class="background-box code">
                <h2 class="connect-heading">"Example code"</h2>
                <pre><code>{code}</code></pre>
            </div>
        </article>
    }
}
