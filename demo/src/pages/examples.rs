use crate::examples::{aspect_sunspots::AspectRatioSunspots, *};
use js_sys::wasm_bindgen::JsCast;
use leptos::{html::Dialog, *};
use leptos_router::{use_location, use_navigate, NavigateOptions};
use web_sys::{HtmlDialogElement, MouseEvent};

macro_rules! example {
    ($id:ident, $ex:path, $title:literal, $desc:literal, $path:literal) => {
        #[component]
        fn $id(
            #[prop(optional, into)] class: Option<AttributeValue>,
            #[prop(optional, into)] data: Option<Signal<Vec<MyData>>>,
        ) -> impl IntoView {
            let ctx = use_local_context();
            let id = title_to_id($title);
            let data = data.unwrap_or(ctx.data);
            view! {
                <figure class=class class:background-box=true>
                    <figcaption>
                        <h3 id=&id><a href=format!("#{id}")>$title</a></h3>
                        <p>$desc " " <ShowCode id=id code=include_str!($path) /></p>
                    </figcaption>
                    <$ex debug=ctx.debug.into() data=data />
                </figure>
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

// Bars
example!(
    BarExample,
    series_bar::Example,
    "Bar chart",
    "A simple bar chart.",
    "../examples/series_bar.rs"
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

// Interpolation

example!(
    MixedInterpolationExample,
    interpolation_mixed::Example,
    "Linear and monotone",
    "Change the interpolation of your lines.",
    "../examples/interpolation_mixed.rs"
);

example!(
    SteppedExample,
    interpolation_stepped::Example,
    "Stepped",
    "Change the interpolation of your lines to stepped.",
    "../examples/interpolation_stepped.rs"
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

example!(
    MarkersExample,
    feature_markers::Example,
    "Point markers",
    "Add point markers to your lines.",
    "../examples/feature_markers.rs"
);

example!(
    Markers2Example,
    feature_markers_2::Example,
    "Point markers 2",
    "Another way to add point markers to your lines.",
    "../examples/feature_markers_2.rs"
);

example!(
    LineGradientExample,
    feature_line_gradient::Example,
    "Line colour scheme",
    "Adds a Y-based gradient to the line colour.",
    "../examples/feature_line_gradient.rs"
);

example!(
    CssExample,
    feature_css::Example,
    "CSS styles",
    "Apply CSS styles to your chart.",
    "../examples/feature_css.rs"
);

#[derive(Clone)]
struct Context {
    debug: RwSignal<bool>,
    data: Signal<Vec<MyData>>,
}

fn use_local_context() -> Context {
    use_context::<Context>().expect("missing examples::context")
}

#[component]
pub fn Examples() -> impl IntoView {
    let debug = create_rw_signal(false);
    provide_context(Context {
        debug,
        data: load_data(),
    });

    view! {
        <article id="examples">
            <div class="cards">
                <nav>
                    <h1>"Examples"</h1>
                    <p class="background-box">
                        <label>
                            <input type="checkbox" input type="checkbox"
                                on:input=move |ev| debug.set(event_target_checked(&ev)) />
                            " Debug mode"
                        </label>
                    </p>
                    <ul class="background-box">
                        <li><a href="#edge">"Edge layout options"</a></li>
                        <li><a href="#inner">"Inner layout options"</a></li>
                        <li><a href="#features">"Features"</a></li>
                    </ul>
                </nav>

                <LineExample />
                <StackedLineExample />

                <div class="include-right">
                    <h2 id="bar"><a href="#bar">"Bar charts"</a></h2>
                    <BarExample />
                </div>

                <h2 id="scatter">"Scatter charts: " <em>"planned"</em></h2>

                <div class="include-right">
                    <h2 id="edge"><a href="#edge">"Edge layout options"</a></h2>
                    <LegendExample />
                </div>
                <TickLabelsExample />
                <RotatedLabelExample />
                <EdgeLayoutExample />

                <div class="include-right">
                    <h2 id="inner"><a href="#inner">"Inner layout options"</a></h2>
                    <AxisMarkerExample />
                </div>
                <GridLineExample />
                <GuideLineExample />
                <InsetLegendExample />
                <InnerLayoutExample />

                <div class="include-right">
                    <h2 id="interpolation"><a href="#interpolation">"Line interpolation"</a></h2>
                    <MixedInterpolationExample />
                </div>
                <SteppedExample />

                <div class="include-right">
                    <h2 id="features"><a href="#features">"Features"</a></h2>
                    <TooltipExample />
                </div>
                <ColoursExample />
                <LineGradientExample class="slim" />
                <MarkersExample />
                <Markers2Example />
                <CssExample class="my-theme" />
            </div>

            <section id="aspect-ratio" class="background-box">
                <h2><a href="#aspect-ratio">"Aspect ratio"</a></h2>
                <AspectRatioSunspots debug=debug.into() />
                <p><ShowCode id="aspect-ratio" code=include_str!("../examples/aspect_sunspots.rs") /></p>
            </section>
        </article>
    }
}

fn title_to_id(title: &str) -> String {
    // ID should not result in any encoding
    title.to_lowercase().replace(' ', "-")
}

#[component]
fn ShowCode(#[prop(into)] id: String, #[prop(into)] code: String) -> impl IntoView {
    let dialog = create_node_ref::<Dialog>();
    let href = format!("#{}", id);

    // Opens dialogue on demand
    let show_modal = move |dialog: HtmlElement<Dialog>| {
        dialog
            .show_modal()
            .expect("unable to show example code dialog")
    };

    let on_click = move |_| {
        dialog.get().map(show_modal);
    };
    let on_dismiss = move |ev: MouseEvent| {
        if let Some(dialog) = dialog.get() {
            if let Some(target) = ev.target() {
                // Skip if click was inside the dialog
                if target.dyn_ref::<HtmlDialogElement>().is_some() {
                    dialog.close();
                    // Navigate away from the fragment
                    use_navigate()(
                        &use_location().pathname.get(),
                        NavigateOptions {
                            resolve: true,
                            replace: true,
                            scroll: false,
                            state: Default::default(),
                        },
                    );
                }
            }
        }
    };

    // Show if page fragment matches ID
    // TODO investigate why this triggers a panic https://docs.rs/leptos/latest/leptos/struct.NodeRef.html#method.on_load
    create_render_effect(move |_| {
        if let Some(dialog) = dialog.get() {
            let hash = use_location().hash.get().trim_start_matches('#').to_owned();
            let hash: String = js_sys::decode_uri(&hash)
                .map(|s| s.into())
                .unwrap_or_default();
            if hash == id {
                let _ = dialog.on_mount(show_modal);
            }
        }
    });

    view! {
        <a href=href on:click=on_click>"Show example code"</a>
        <dialog node_ref=dialog on:click=on_dismiss>
            <pre><code>{code}</code></pre>
        </dialog>
    }
}
