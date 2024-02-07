use crate::examples::*;
use js_sys::wasm_bindgen::JsCast;
use leptos::{html::Dialog, *};
use web_sys::{HtmlDialogElement, MouseEvent};

macro_rules! figure {
    ($id:literal, $title:literal, $desc:literal, $path:literal) => {
        view! {
            <figure id=$id class="background-box">
                <figcaption>
                    <h3>$title</h3>
                    <p>$desc " " <ShowCode code=include_str!($path) /></p>
                </figcaption>
                <edge_legend::Example data=load_data() />
            </figure>
        }
    };
}

fn all_example_figures() -> Vec<(&'static str, Vec<impl IntoView>)> {
    vec![(
        "Edge layout options",
        vec![figure!(
            "edge-legend",
            "Legend",
            "Add legends to your chart.",
            "../examples/edge_legend.rs"
        )],
    )]
}

#[component]
pub fn Examples() -> impl IntoView {
    let all_figures = all_example_figures()
        .into_iter()
        .map(|(header, figures)| {
            view! {
                <h2 id="todo">{header}</h2>
                <div class="cards">
                    {figures.into_iter().collect_view()}
                </div>
            }
        })
        .collect_view();

    view! {
        <article id="examples">
            <h1>"Examples"</h1>
            <nav>
                <ul class="background-box">
                    <li>
                        <a href="#series">"By chart series"</a>": "
                        <ul>
                            <li><a href="#series-line">"Line charts"</a></li>
                            <li><a href="#series-bar">"Bar charts"</a></li>
                            <li><a href="#series-scatter">"Scatter charts"</a></li>
                        </ul>
                    </li>
                    <li>
                        <a href="#edge">"By edge layout"</a>": "
                        <ul>
                            <li><a href="#edge-legend">"Legend"</a></li>
                            <li><a href="#edge-text">"Text label"</a></li>
                            <li><a href="#edge-ticks">"Tick labels"</a></li>
                        </ul>
                    </li>
                    <li>
                        <a href="#inner">"By inner layout"</a>": "
                        <ul>
                            <li><a href="#inner-axis">"Axis marker"</a></li>
                            <li><a href="#inner-grid">"Grid line"</a></li>
                            <li><a href="#inner-guide">"Guide line"</a></li>
                            <li><a href="#inner-legend">"Legend"</a></li>
                        </ul>
                    </li>
                    <li>
                        <a href="#feature">"By feature"</a>": "
                        <ul>
                            <li><a href="#feature-colour">"Colours"</a></li>
                            <li><a href="#feature-width">"Line widths"</a></li>
                        </ul>
                    </li>
                </ul>
            </nav>

            <div id="series">
                <div id="series-line">
                    <h2>"Line charts"</h2>
                    <div class="card">
                        "todo"
                    </div>
                </div>

                <div id="series-bar">
                    <h2>"Bar charts"</h2>
                    <p>"Planned"</p>
                </div>

                <div id="series-scatter">
                    <h2>"Scatter charts"</h2>
                    <p>"Planned"</p>
                </div>
            </div>

            {all_figures}

        </article>
    }
}

#[component]
fn ShowCode(#[prop(into)] code: String) -> impl IntoView {
    let dialog = create_node_ref::<Dialog>();
    let on_open = move |ev: MouseEvent| {
        ev.prevent_default();
        if let Some(dialog) = dialog.get() {
            dialog
                .show_modal()
                .expect("unable to show example code dialog");
        }
    };
    let on_close = move |ev: MouseEvent| {
        ev.prevent_default();
        if let Some(dialog) = dialog.get() {
            // Close dialogue (it covers the whole page) on interaction unless user clicks on text inside
            if let Some(target) = ev.target() {
                if target.dyn_ref::<HtmlDialogElement>().is_some() {
                    dialog.close()
                }
            }
        }
    };
    view! {
        <a href="#" on:click=on_open>"Show example code"</a>
        <dialog node_ref=dialog on:click=on_close>
            <pre><code>{code}</code></pre>
        </dialog>
    }
}
