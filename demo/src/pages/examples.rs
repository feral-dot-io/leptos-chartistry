use crate::{
    examples::{aspect_sunspots::AspectRatioSunspots, *},
    use_app_context,
};
use js_sys::wasm_bindgen::JsCast;
use leptos::{html::Dialog, *};
use leptos_router::{use_location, use_navigate, NavigateOptions};
use strum::VariantArray;
use web_sys::{HtmlDialogElement, MouseEvent};

#[derive(Copy, Clone, Debug, PartialEq, VariantArray)]
pub enum Example {
    Line,
    StackedLine,
    Bar,
    Legend,
    TickLabels,
    RotatedLabel,
    EdgeLayout,
    AxisMarker,
    GridLine,
    GuideLine,
    InsetLegend,
    InnerLayout,
    MixedInterpolation,
    Stepped,
    Tooltip,
    Colours,
    Markers,
    Markers2,
    LineGradient,
    Css,
}

impl Example {
    fn title(self) -> &'static str {
        match self {
            Self::Line => "Line chart",
            Self::StackedLine => "Stacked line chart",
            Self::Bar => "Bar chart",
            Self::Legend => "Legend",
            Self::TickLabels => "Tick labels",
            Self::RotatedLabel => "Rotated label",
            Self::EdgeLayout => "Combined edge layout",
            Self::AxisMarker => "Axis marker",
            Self::GridLine => "Grid line",
            Self::GuideLine => "Guide line",
            Self::InsetLegend => "Inset legend",
            Self::InnerLayout => "Combined inner layout",
            Self::MixedInterpolation => "Linear and monotone",
            Self::Stepped => "Stepped",
            Self::Tooltip => "Tooltip",
            Self::Colours => "Colour",
            Self::Markers => "Point markers",
            Self::Markers2 => "Point markers 2",
            Self::LineGradient => "Line colour scheme",
            Self::Css => "CSS styles",
        }
    }

    fn id(self) -> String {
        // ID should not result in any encoding in a URL
        self.title().to_lowercase().replace(' ', "-")
    }

    fn description(self) -> &'static str {
        match self {
            Self::Line => "A simple line chart.",
            Self::StackedLine => "A stacked line chart.",
            Self::Bar => "A simple bar chart.",
            Self::Legend => "Add legends to your chart edges.",
            Self::TickLabels => "Add tick labels and auto-pick nice values.",
            Self::RotatedLabel => "Add rotated labels to your chart.",
            Self::EdgeLayout => "A more complete example of all edge options.",
            Self::AxisMarker => "Add axis markers to the edges of your chart area.",
            Self::GridLine => "Add grid lines aligned to your tick labels.",
            Self::GuideLine => "Add guide lines to your mouse.",
            Self::InsetLegend => "Add a legend inside your chart area.",
            Self::InnerLayout => "A more complete example of all inner options.",
            Self::MixedInterpolation => "Change the interpolation of your lines.",
            Self::Stepped => "Change the interpolation of your lines to stepped.",
            Self::Tooltip => "Add a mouse tooltip to your chart.",
            Self::Colours => "Change the colours of your chart.",
            Self::Markers => "Add point markers to your lines.",
            Self::Markers2 => "Another way to add point markers to your lines.",
            Self::LineGradient => "Adds a Y-based gradient to the line colour.",
            Self::Css => "Apply CSS styles to your chart.",
        }
    }

    fn code(self) -> &'static str {
        match self {
            Self::Line => include_str!("../examples/series_line.rs"),
            Self::StackedLine => include_str!("../examples/series_line_stack.rs"),
            Self::Bar => include_str!("../examples/series_bar.rs"),
            Self::Legend => include_str!("../examples/edge_legend.rs"),
            Self::TickLabels => include_str!("../examples/edge_tick_labels.rs"),
            Self::RotatedLabel => include_str!("../examples/edge_rotated_label.rs"),
            Self::EdgeLayout => include_str!("../examples/edge_layout.rs"),
            Self::AxisMarker => include_str!("../examples/inner_axis_marker.rs"),
            Self::GridLine => include_str!("../examples/inner_grid_line.rs"),
            Self::GuideLine => include_str!("../examples/inner_guide_line.rs"),
            Self::InsetLegend => include_str!("../examples/inner_legend.rs"),
            Self::InnerLayout => include_str!("../examples/inner_layout.rs"),
            Self::MixedInterpolation => include_str!("../examples/interpolation_mixed.rs"),
            Self::Stepped => include_str!("../examples/interpolation_stepped.rs"),
            Self::Tooltip => include_str!("../examples/feature_tooltip.rs"),
            Self::Colours => include_str!("../examples/feature_colours.rs"),
            Self::Markers => include_str!("../examples/feature_markers.rs"),
            Self::Markers2 => include_str!("../examples/feature_markers_2.rs"),
            Self::LineGradient => include_str!("../examples/feature_line_gradient.rs"),
            Self::Css => include_str!("../examples/feature_css.rs"),
        }
    }

    fn extra_class(self) -> Option<AttributeValue> {
        match self {
            Self::LineGradient => Some("slim".into()),
            Self::Css => Some("my-theme".into()),
            _ => None,
        }
    }

    fn card_view(self) -> impl IntoView {
        let de = use_app_context().debug.into();
        let da = load_data();
        match self {
            Self::Line => view!(<series_line::Example debug=de data=da />),
            Self::StackedLine => view!(<series_line_stack::Example debug=de data=da />),
            Self::Bar => view!(<series_bar::Example debug=de data=da />),
            Self::Legend => view!(<edge_legend::Example debug=de data=da />),
            Self::TickLabels => view!(<edge_tick_labels::Example debug=de data=da />),
            Self::RotatedLabel => view!(<edge_rotated_label::Example debug=de data=da />),
            Self::EdgeLayout => view!(<edge_layout::Example debug=de data=da />),
            Self::AxisMarker => view!(<inner_axis_marker::Example debug=de data=da />),
            Self::GridLine => view!(<inner_grid_line::Example debug=de data=da />),
            Self::GuideLine => view!(<inner_guide_line::Example debug=de data=da />),
            Self::InsetLegend => view!(<inner_legend::Example debug=de data=da />),
            Self::InnerLayout => view!(<inner_layout::Example debug=de data=da />),
            Self::MixedInterpolation => view!(<interpolation_mixed::Example debug=de data=da />),
            Self::Stepped => view!(<interpolation_stepped::Example debug=de data=da />),
            Self::Tooltip => view!(<feature_tooltip::Example debug=de data=da />),
            Self::Colours => view!(<feature_colours::Example debug=de data=da />),
            Self::Markers => view!(<feature_markers::Example debug=de data=da />),
            Self::Markers2 => view!(<feature_markers_2::Example debug=de data=da />),
            Self::LineGradient => view!(<feature_line_gradient::Example debug=de data=da />),
            Self::Css => view!(<feature_css::Example debug=de data=da />),
        }
    }

    fn page_view(self) -> impl IntoView {
        // Note: page-specific variants that do more than just the card view should be referenced here. Falls back to the card view if not implemented.
        view! {
            <Example example=self />
        }
    }
}

#[component(transparent)]
pub fn Routes(prefix: &'static str) -> impl IntoView {
    use leptos_router::*;
    // Note: this was incredibly awkward and fiddly to get right. The `Route::children` attribute requires a `Fragment` built from a `Vec<View>`. The `View` must be made up of a transparent `Route` component. It seems any deviation from this e.g., using `CollectView` results in a "tried to mount a Transparent node." error from Leptos.
    let children = Example::VARIANTS
        .iter()
        .map(|ex| {
            view! {
                <Route path=format!("{}.html", ex.id()) view=|| ex.page_view() />
            }
        })
        .map(|r| r.into_view())
        .collect::<Fragment>();
    view! {
        <Route path=prefix view=|| view!(<Outlet />) children=Box::new(|| children) />
    }
}

#[component]
fn Card(example: Example, #[prop(optional)] h1: bool) -> impl IntoView {
    let id = example.id();
    let url = format!("examples/{id}.html");
    let heading = if h1 {
        view!( <h1 id=&id><a href=&url>{example.title()}</a></h1> ).into_view()
    } else {
        view!( <h3 id=&id><a href=&url>{example.title()}</a></h3> ).into_view()
    };
    view! {
        <figure class:background-box=true class=example.extra_class()>
            <figcaption>
                {heading}
                <p>{example.description()} " " <a href=&url>"Show example code"</a></p>
            </figcaption>
            {example.card_view()}
        </figure>
    }
}

#[component]
pub fn Example(example: Example) -> impl IntoView {
    let app = use_app_context();
    view! {
        <article class="example">
            <div class="cards">
                <Card example=example h1=true />
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
                <pre><code>{example.code()}</code></pre>
            </div>
        </article>
    }
}

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

                <Card example=Example::Line />
                <Card example=Example::StackedLine />

                <div class="include-right">
                    <h2 id="bar"><a href="#bar">"Bar charts"</a></h2>
                    <Card example=Example::Bar />
                </div>

                <div class="include-right">
                    <h2 id="edge"><a href="#edge">"Edge layout options"</a></h2>
                    <Card example=Example::Legend />
                </div>
                <Card example=Example::TickLabels />
                <Card example=Example::RotatedLabel />
                <Card example=Example::EdgeLayout />

                <div class="include-right">
                    <h2 id="inner"><a href="#inner">"Inner layout options"</a></h2>
                    <Card example=Example::AxisMarker />
                </div>
                <Card example=Example::GridLine />
                <Card example=Example::GuideLine />
                <Card example=Example::InsetLegend />
                <Card example=Example::InnerLayout />

                <div class="include-right">
                    <h2 id="interpolation"><a href="#interpolation">"Line interpolation"</a></h2>
                    <Card example=Example::MixedInterpolation />
                </div>
                <Card example=Example::Stepped />

                <div class="include-right">
                    <h2 id="features"><a href="#features">"Features"</a></h2>
                    <Card example=Example::Tooltip />
                </div>
                <Card example=Example::Colours />
                <Card example=Example::LineGradient />
                <Card example=Example::Markers />
                <Card example=Example::Markers2 />
                <Card example=Example::Css />
            </div>

            <section id="aspect-ratio" class="background-box">
                <h2><a href="#aspect-ratio">"Aspect ratio"</a></h2>
                <AspectRatioSunspots debug=app.debug.into() />
                <p><ShowCode id="aspect-ratio" code=include_str!("../examples/aspect_sunspots.rs") /></p>
            </section>
        </article>
    }
}

#[component]
fn ShowCode(#[prop(into)] id: String, #[prop(into)] code: String) -> impl IntoView {
    let dialog = create_node_ref::<Dialog>();
    let href = format!("examples.html#{}", id);

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
