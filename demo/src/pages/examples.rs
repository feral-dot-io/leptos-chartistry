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

// Lines

example!(
    LineExample,
    series_line::Example,
    "Line chart",
    "A simple line chart.",
    "../examples/series_line.rs"
);

example!(
    StackedLineExample,
    series_line_stack::Example,
    "Stacked line chart",
    "A stacked line chart.",
    "../examples/series_line_stack.rs"
);

// Edge layout options
example!(
    LegendExample,
    edge_legend::Example,
    "Legend",
    "Add legends to your chart edges.",
    "../examples/edge_legend.rs"
);

example!(
    TickLabelsExample,
    edge_tick_labels::Example,
    "Tick labels",
    "Add tick labels and auto-pick nice values.",
    "../examples/edge_tick_labels.rs"
);

example!(
    RotatedLabelExample,
    edge_rotated_label::Example,
    "Rotated label",
    "Add rotated labels to your chart.",
    "../examples/edge_rotated_label.rs"
);

example!(
    EdgeLayoutExample,
    edge_layout::Example,
    "Combined edge layout",
    "A more complete example of all edge options.",
    "../examples/edge_layout.rs"
);

// Inner layout options
example!(
    AxisMarkerExample,
    inner_axis_marker::Example,
    "Axis marker",
    "Add axis markers to the edges of your chart area.",
    "../examples/inner_axis_marker.rs"
);

example!(
    GridLineExample,
    inner_grid_line::Example,
    "Grid line",
    "Add grid lines aligned to your tick labels.",
    "../examples/inner_grid_line.rs"
);

example!(
    GuideLineExample,
    inner_guide_line::Example,
    "Guide line",
    "Add guide lines to your mouse.",
    "../examples/inner_guide_line.rs"
);

example!(
    InsetLegendExample,
    inner_legend::Example,
    "Inset legend",
    "Add a legend inside your chart area.",
    "../examples/inner_legend.rs"
);

example!(
    InnerLayoutExample,
    inner_layout::Example,
    "Combined inner layout",
    "A more complete example of all inner options.",
    "../examples/inner_layout.rs"
);

// Features

example!(
    TooltipExample,
    feature_tooltip::Example,
    "Tooltip",
    "Add a mouse tooltip to your chart.",
    "../examples/feature_tooltip.rs"
);

example!(
    ColoursExample,
    feature_colours::Example,
    "Colour",
    "Change the colours of your chart.",
    "../examples/feature_colours.rs"
);

#[component]
pub fn Examples() -> impl IntoView {
    let (debug, set_debug) = create_signal(false);
    let data = load_data();
    view! {
        <article id="examples">
            <h1>"Examples"</h1>
            <div class="cards">
                <nav>
                    <p class="background-box">
                        <label>
                            <input type="checkbox" input type="checkbox"
                                on:input=move |ev| set_debug.set(event_target_checked(&ev)) />
                            " Debug mode"
                        </label>
                    </p>
                    <ul class="background-box">
                        <li><a href="#line">"Line charts"</a></li>
                        <li><a href="#edge">"Edge layout options"</a></li>
                        <li><a href="#inner">"Inner layout options"</a></li>
                        <li><a href="#features">"Features"</a></li>
                    </ul>
                </nav>

                <h2 id="line">"Line charts"</h2>
                <LineExample debug=debug data=data />
                <StackedLineExample debug=debug data=data />

                <h2 id="bar">"Bar charts: " <em>"planned"</em></h2>
                <h2 id="scatter">"Scatter charts: " <em>"planned"</em></h2>

                <h2 id="edge">"Edge layout options"</h2>
                <LegendExample debug=debug data=data />
                <TickLabelsExample debug=debug data=data />
                <RotatedLabelExample debug=debug data=data />
                <EdgeLayoutExample debug=debug data=data />

                <h2 id="inner">"Inner layout options"</h2>
                <AxisMarkerExample debug=debug data=data />
                <GridLineExample debug=debug data=data />
                <GuideLineExample debug=debug data=data />
                <InsetLegendExample debug=debug data=data />
                <InnerLayoutExample debug=debug data=data />

                <h2 id="features">"Features"</h2>
                <TooltipExample debug=debug data=data />
                <ColoursExample debug=debug data=data />
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
