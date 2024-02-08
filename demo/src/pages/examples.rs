use crate::examples::*;
use js_sys::wasm_bindgen::JsCast;
use leptos::{html::Dialog, *};
use web_sys::{HtmlDialogElement, MouseEvent};

#[component]
fn ExampleFigure(
    title: &'static str,
    desc: &'static str,
    code: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <figure class="background-box">
            <figcaption>
                <h3>{title}</h3>
                <p>{desc} " " <ShowCode code=code /></p>
            </figcaption>
            {children()}
        </figure>
    }
}

macro_rules! example {
    ($id:ident, $ex:path, $title:literal, $desc:literal, $path:literal) => {
        #[component]
        fn $id(debug: ReadSignal<bool>, data: Signal<Vec<MyData>>) -> impl IntoView {
            view! {
                <ExampleFigure title=$title desc=$desc code=include_str!($path)>
                    <$ex debug=debug.into() data=data />
                </ExampleFigure>
            }
        }
    };
}

// Edge layout options
example!(
    LegendExample,
    edge_legend::Example,
    "Legend",
    "Add legends to your chart.",
    "../examples/edge_legend.rs"
);

example!(
    TickLabelsExample,
    edge_tick_labels::Example,
    "Tick labels",
    "Add tick labels to your chart.",
    "../examples/edge_tick_labels.rs"
);

example!(
    RotatedLabelExample,
    edge_rotated_label::Example,
    "Rotated label",
    "Add rotated labels to your chart.",
    "../examples/edge_rotated_label.rs"
);

// Inner layout options
example!(
    AxisMarkerExample,
    inner_axis_marker::Example,
    "Axis marker",
    "Add axis markers to your chart.",
    "../examples/inner_axis_marker.rs"
);

example!(
    InsetLegendExample,
    inner_legend::Example,
    "Inset legend",
    "Add an inset legend to your chart.",
    "../examples/inner_legend.rs"
);

#[component]
pub fn Examples() -> impl IntoView {
    let (debug, set_debug) = create_signal(false);
    let data = load_data();
    view! {
        <article id="examples">
            <h1>"Examples"</h1>

            <p>
                <label>
                    <input type="checkbox" input type="checkbox"
                        on:input=move |ev| set_debug.set(event_target_checked(&ev)) />
                    " Debug mode"
                </label>
            </p>

            /*
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
            */

            <h2 id="edge">"Edge layout options"</h2>
            <div class="cards">
                <LegendExample debug=debug data=data />
                <TickLabelsExample debug=debug data=data />
                <RotatedLabelExample debug=debug data=data />
            </div>


            <h2 id="inner">"Inner layout options"</h2>
            <div class="cards">
                <AxisMarkerExample debug=debug data=data />
                <InsetLegendExample debug=debug data=data />
            </div>


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