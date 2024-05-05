use crate::{
    examples::{aspect_sunspots::AspectRatioSunspots, *},
    use_app_context,
};
use js_sys::wasm_bindgen::JsCast;
use leptos::{html::Dialog, *};
use leptos_router::{use_location, use_navigate, NavigateOptions};
use web_sys::{HtmlDialogElement, MouseEvent};

macro_rules! example {
    ($ex:path, $card:ident, $page:ident, $title:literal, $desc:literal, $path:literal) => {
        #[component]
        fn $card(
            #[prop(optional)] h1: bool,
            #[prop(optional, into)] class: Option<AttributeValue>,
        ) -> impl IntoView {
            let app = use_app_context();
            let id = title_to_id($title);
            let url = format!("examples/{id}.html");
            let heading = if h1 {
                view!( <h1 id=&id><a href=&url>$title</a></h1> ).into_view()
            } else {
                view!( <h3 id=&id><a href=&url>$title</a></h3> ).into_view()
            };
            view! {
                <figure class=class class:background-box=true>
                    <figcaption>
                        {heading}
                        <p>$desc " " <a href=&url>"Show example code"</a></p>
                    </figcaption>
                    <$ex debug=app.debug.into() data=load_data() />
                </figure>
            }
        }

        #[component]
        pub fn $page() -> impl IntoView {
            let app = use_app_context();
            let code = include_str!($path);
            view! {
                <article class="example">
                    <div class="cards">
                        <$card h1=true />
                        <div class="background-box debug">
                            <label>
                                <input type="checkbox" type="checkbox" prop:checked=app.debug
                                    on:input=move |ev| app.debug.set(event_target_checked(&ev)) />
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
    };
}

// Lines
example!(
    series_line::Example,
    LineExampleCard,
    LineExamplePage,
    "Line chart",
    "A simple line chart.",
    "../examples/series_line.rs"
);

example!(
    series_line_stack::Example,
    StackedLineExampleCard,
    StackedLineExamplePage,
    "Stacked line chart",
    "A stacked line chart.",
    "../examples/series_line_stack.rs"
);

// Bars
example!(
    series_bar::Example,
    BarExampleCard,
    BarExamplePage,
    "Bar chart",
    "A simple bar chart.",
    "../examples/series_bar.rs"
);

// Edge layout options
example!(
    edge_legend::Example,
    LegendExampleCard,
    LegendExamplePage,
    "Legend",
    "Add legends to your chart edges.",
    "../examples/edge_legend.rs"
);

example!(
    edge_tick_labels::Example,
    TickLabelsExampleCard,
    TickLabelsExamplePage,
    "Tick labels",
    "Add tick labels and auto-pick nice values.",
    "../examples/edge_tick_labels.rs"
);

example!(
    edge_rotated_label::Example,
    RotatedLabelExampleCard,
    RotatedLabelExamplePage,
    "Rotated label",
    "Add rotated labels to your chart.",
    "../examples/edge_rotated_label.rs"
);

example!(
    edge_layout::Example,
    EdgeLayoutExampleCard,
    EdgeLayoutExamplePage,
    "Combined edge layout",
    "A more complete example of all edge options.",
    "../examples/edge_layout.rs"
);

// Inner layout options
example!(
    inner_axis_marker::Example,
    AxisMarkerExampleCard,
    AxisMarkerExamplePage,
    "Axis marker",
    "Add axis markers to the edges of your chart area.",
    "../examples/inner_axis_marker.rs"
);

example!(
    inner_grid_line::Example,
    GridLineExampleCard,
    GridLineExamplePage,
    "Grid line",
    "Add grid lines aligned to your tick labels.",
    "../examples/inner_grid_line.rs"
);

example!(
    inner_guide_line::Example,
    GuideLineExampleCard,
    GuideLineExamplePage,
    "Guide line",
    "Add guide lines to your mouse.",
    "../examples/inner_guide_line.rs"
);

example!(
    inner_legend::Example,
    InsetLegendExampleCard,
    InsetLegendExamplePage,
    "Inset legend",
    "Add a legend inside your chart area.",
    "../examples/inner_legend.rs"
);

example!(
    inner_layout::Example,
    InnerLayoutExampleCard,
    InnerLayoutExamplePage,
    "Combined inner layout",
    "A more complete example of all inner options.",
    "../examples/inner_layout.rs"
);

// Interpolation

example!(
    interpolation_mixed::Example,
    MixedInterpolationExampleCard,
    MixedInterpolationExamplePage,
    "Linear and monotone",
    "Change the interpolation of your lines.",
    "../examples/interpolation_mixed.rs"
);

example!(
    interpolation_stepped::Example,
    SteppedExampleCard,
    SteppedExamplePage,
    "Stepped",
    "Change the interpolation of your lines to stepped.",
    "../examples/interpolation_stepped.rs"
);

// Features

example!(
    feature_tooltip::Example,
    TooltipExampleCard,
    TooltipExamplePage,
    "Tooltip",
    "Add a mouse tooltip to your chart.",
    "../examples/feature_tooltip.rs"
);

example!(
    feature_colours::Example,
    ColoursExampleCard,
    ColoursExamplePage,
    "Colour",
    "Change the colours of your chart.",
    "../examples/feature_colours.rs"
);

example!(
    feature_markers::Example,
    MarkersExampleCard,
    MarkersExamplePage,
    "Point markers",
    "Add point markers to your lines.",
    "../examples/feature_markers.rs"
);

example!(
    feature_markers_2::Example,
    Markers2ExampleCard,
    Markers2ExamplePage,
    "Point markers 2",
    "Another way to add point markers to your lines.",
    "../examples/feature_markers_2.rs"
);

example!(
    feature_line_gradient::Example,
    LineGradientExampleCard,
    LineGradientExamplePage,
    "Line colour scheme",
    "Adds a Y-based gradient to the line colour.",
    "../examples/feature_line_gradient.rs"
);

example!(
    feature_css::Example,
    CssExampleCard,
    CssExamplePage,
    "CSS styles",
    "Apply CSS styles to your chart.",
    "../examples/feature_css.rs"
);

#[component]
pub fn Examples() -> impl IntoView {
    let app = use_app_context();

    view! {
        <article id="examples">
            <div class="cards">
                <nav>
                    <h1>"Examples"</h1>
                    <p class="background-box">
                        <label>
                            <input type="checkbox" type="checkbox" prop:checked=app.debug
                                on:input=move |ev| app.debug.set(event_target_checked(&ev)) />
                            " Debug mode"
                        </label>
                    </p>
                    <ul class="background-box">
                        <li><a href="#edge">"Edge layout options"</a></li>
                        <li><a href="#inner">"Inner layout options"</a></li>
                        <li><a href="#features">"Features"</a></li>
                    </ul>
                </nav>

                <LineExampleCard />
                <StackedLineExampleCard />

                <div class="include-right">
                    <h2 id="bar"><a href="#bar">"Bar charts"</a></h2>
                    <BarExampleCard />
                </div>

                <div class="include-right">
                    <h2 id="edge"><a href="#edge">"Edge layout options"</a></h2>
                    <LegendExampleCard />
                </div>
                <TickLabelsExampleCard />
                <RotatedLabelExampleCard />
                <EdgeLayoutExampleCard />

                <div class="include-right">
                    <h2 id="inner"><a href="#inner">"Inner layout options"</a></h2>
                    <AxisMarkerExampleCard />
                </div>
                <GridLineExampleCard />
                <GuideLineExampleCard />
                <InsetLegendExampleCard />
                <InnerLayoutExampleCard />

                <div class="include-right">
                    <h2 id="interpolation"><a href="#interpolation">"Line interpolation"</a></h2>
                    <MixedInterpolationExampleCard />
                </div>
                <SteppedExampleCard />

                <div class="include-right">
                    <h2 id="features"><a href="#features">"Features"</a></h2>
                    <TooltipExampleCard />
                </div>
                <ColoursExampleCard />
                <LineGradientExampleCard class="slim" />
                <MarkersExampleCard />
                <Markers2ExampleCard />
                <CssExampleCard class="my-theme" />
            </div>

            <section id="aspect-ratio" class="background-box">
                <h2><a href="#aspect-ratio">"Aspect ratio"</a></h2>
                <AspectRatioSunspots debug=app.debug.into() />
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
