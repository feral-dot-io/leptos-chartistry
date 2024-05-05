use crate::examples::{load_data, series_line};
use leptos::*;

#[component]
pub fn Example() -> impl IntoView {
    let debug = create_rw_signal(false);
    let data = load_data();
    let code = include_str!("../../examples/series_line.rs");
    view! {
        <article class="example">
            <div class="cards">
                <figure class="background-box">
                    <figcaption>
                        <h1 id="line-chart"><a href="line-chart.html">"Line chart"</a></h1>
                        <p>"A simple line chart. "</p>
                    </figcaption>
                    <series_line::Example debug=debug.into() data=data />
                </figure>
                <div class="background-box debug">
                    <label>
                        <input type="checkbox" input type="checkbox"
                            on:input=move |ev| debug.set(event_target_checked(&ev)) />
                        " Toggle debug mode"
                    </label>
                </div>
            </div>
            <div class="background-box code">
                <h2 class="connect-heading">"Example code"</h2>
                <pre><code>{code}</code></pre>
            </div>
        </article>
    }
}
