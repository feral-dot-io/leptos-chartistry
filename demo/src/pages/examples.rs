use crate::{
    examples::{aspect_sunspots::AspectRatioSunspots, *},
    use_app_context,
};
use leptos::{
    either::{Either},
    prelude::*,
};
use strum::VariantArray;

#[derive(Copy, Clone, Debug, PartialEq, VariantArray)]
pub enum Example {
    // Note: changes here requires an update to routes and impl_example_view
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

macro_rules! include_example_hl {
    ($file:tt) => {
        include_str!(concat!(env!("OUT_DIR"), "/examples-hl/", $file, ".rs"))
    };
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

    pub const fn id(self) -> &'static str {
        match self {
            Self::Line => "series-line",
            Self::StackedLine => "series-line-stack",
            Self::Bar => "series-bar",
            Self::Legend => "edge-legend",
            Self::TickLabels => "edge-tick-labels",
            Self::RotatedLabel => "edge-rotated-label",
            Self::EdgeLayout => "edge-layout",
            Self::AxisMarker => "inner-axis-marker",
            Self::GridLine => "inner-grid-line",
            Self::GuideLine => "inner-guide-line",
            Self::InsetLegend => "inner-legend",
            Self::InnerLayout => "inner-layout",
            Self::MixedInterpolation => "interpolation-mixed",
            Self::Stepped => "interpolation-stepped",
            Self::Tooltip => "feature-tooltip",
            Self::Colours => "feature-colours",
            Self::Markers => "feature-markers",
            Self::Markers2 => "feature-markers-2",
            Self::LineGradient => "feature-line-gradient",
            Self::Css => "feature-css",
        }
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
            Self::Line => include_example_hl!("series_line"),
            Self::StackedLine => include_example_hl!("series_line_stack"),
            Self::Bar => include_example_hl!("series_bar"),
            Self::Legend => include_example_hl!("edge_legend"),
            Self::TickLabels => include_example_hl!("edge_tick_labels"),
            Self::RotatedLabel => include_example_hl!("edge_rotated_label"),
            Self::EdgeLayout => include_example_hl!("edge_layout"),
            Self::AxisMarker => include_example_hl!("inner_axis_marker"),
            Self::GridLine => include_example_hl!("inner_grid_line"),
            Self::GuideLine => include_example_hl!("inner_guide_line"),
            Self::InsetLegend => include_example_hl!("inner_legend"),
            Self::InnerLayout => include_example_hl!("inner_layout"),
            Self::MixedInterpolation => include_example_hl!("interpolation_mixed"),
            Self::Stepped => include_example_hl!("interpolation_stepped"),
            Self::Tooltip => include_example_hl!("feature_tooltip"),
            Self::Colours => include_example_hl!("feature_colours"),
            Self::Markers => include_example_hl!("feature_markers"),
            Self::Markers2 => include_example_hl!("feature_markers_2"),
            Self::LineGradient => include_example_hl!("feature_line_gradient"),
            Self::Css => include_example_hl!("feature_css"),
        }
    }

    fn extra_class(self) -> (&'static str, bool) {
        match self {
            Self::LineGradient => ("slim", true),
            Self::Css => ("my-theme", true),
            _ => ("", false),
        }
    }

    fn card_view(self) -> impl IntoView {
        let de = use_app_context().debug.into();
        let da = load_data();
        match self {
            Self::Line => view! {<series_line::Example debug=de data=da />}.into_any(),
            Self::StackedLine => view! {<series_line_stack::Example debug=de data=da />}.into_any(),
            Self::Bar => view! {<series_bar::Example debug=de data=da />}.into_any(),
            Self::Legend => view! {<edge_legend::Example debug=de data=da />}.into_any(),
            Self::TickLabels => view! {<edge_tick_labels::Example debug=de data=da />}.into_any(),
            Self::RotatedLabel => view! {<edge_rotated_label::Example debug=de data=da />}.into_any(),
            Self::EdgeLayout => view! {<edge_layout::Example debug=de data=da />}.into_any(),
            Self::AxisMarker => view! {<inner_axis_marker::Example debug=de data=da />}.into_any(),
            Self::GridLine => view! {<inner_grid_line::Example debug=de data=da />}.into_any(),
            Self::GuideLine => view! {<inner_guide_line::Example debug=de data=da />}.into_any(),
            Self::InsetLegend => view! {<inner_legend::Example debug=de data=da />}.into_any(),
            Self::InnerLayout => view! {<inner_layout::Example debug=de data=da />}.into_any(),
            Self::MixedInterpolation => view! {<interpolation_mixed::Example debug=de data=da />}.into_any(),
            Self::Stepped => view! {<interpolation_stepped::Example debug=de data=da />}.into_any(),
            Self::Tooltip => view! {<feature_tooltip::Example debug=de data=da />}.into_any(),
            Self::Colours => view! {<feature_colours::Example debug=de data=da />}.into_any(),
            Self::Markers => view! {<feature_markers::Example debug=de data=da />}.into_any(),
            Self::Markers2 => view! {<feature_markers_2::Example debug=de data=da />}.into_any(),
            Self::LineGradient => view! {<feature_line_gradient::Example debug=de data=da />}.into_any(),
            Self::Css => view! {<feature_css::Example debug=de data=da />}.into_any(),
        }
    }
}

#[component]
fn Card(example: Example, #[prop(optional)] h1: bool) -> impl IntoView {
    let id = example.id();
    let url = format!("examples/{id}.html");
    let heading = if h1 {
        Either::Left(view! {
            <h1 id=id><a href=url.clone()>{example.title()}</a></h1>
        })
    } else {
        Either::Right(view! {
            <h3 id=id><a href=url.clone()>{example.title()}</a></h3>
        })
    };
    view! {
        <figure class:background-box=true class=example.extra_class()>
            <figcaption>
                {heading}
                <p>{example.description()} " " <a href=url>"Show example code"</a></p>
            </figcaption>
            {example.card_view()}
        </figure>
    }
}

pub fn view_example(example: Example) -> impl IntoView {
    let app = use_app_context();
    view! {
        <article class="example">
            <div class="cards">
                <Card example=example h1=true />
                <div class="background-box debug-box">
                    <label>
                        <input type="checkbox" prop:checked=app.debug
                            on:input=move |ev| app.debug.set(event_target_checked(&ev)) />
                        " Toggle debug mode"
                    </label>
                </div>
            </div>
            <div class="background-box code">
                <div inner_html=example.code() />
            </div>
        </article>
    }
    .into_any()
}

#[component]
pub fn Examples() -> impl IntoView {
    let app = use_app_context();

    view! {
        <article id="examples">
            <p class="debug-box background-box">
                <label>
                    <input type="checkbox" prop:checked=app.debug
                        on:input=move |ev| app.debug.set(event_target_checked(&ev)) />
                    " Toggle debug mode"
                </label>
            </p>

            <div id="line" class="cards-row">
                <h2><a href="examples.html#line">"Charts"</a></h2>
                <div class="cards">
                    <Card example=Example::Line />
                    <Card example=Example::StackedLine />
                    <Card example=Example::Bar />
                </div>
            </div>

            <div id="edge" class="cards-row">
                <h2><a href="examples.html#edge">"Edge layout options"</a></h2>
                <div class="cards">
                    <Card example=Example::Legend />
                    <Card example=Example::TickLabels />
                    <Card example=Example::RotatedLabel />
                    <Card example=Example::EdgeLayout />
                </div>
            </div>

            <div id="inner" class="cards-row">
                <h2><a href="examples.html#inner">"Inner layout options"</a></h2>
                <div class="cards">
                    <Card example=Example::AxisMarker />
                    <Card example=Example::GridLine />
                    <Card example=Example::GuideLine />
                    <Card example=Example::InsetLegend />
                    <Card example=Example::InnerLayout />
                </div>
            </div>

            <div id="interpolation" class="cards-row">
                <h2><a href="examples.html#interpolation">"Line interpolation"</a></h2>
                <div class="cards">
                    <Card example=Example::MixedInterpolation />
                    <Card example=Example::Stepped />
                </div>
            </div>

            <div id="features" class="cards-row">
                <h2><a href="examples.html#features">"Features"</a></h2>
                <div class="cards">
                    <Card example=Example::Tooltip />
                    <Card example=Example::Colours />
                    <Card example=Example::LineGradient />
                    <Card example=Example::Markers />
                    <Card example=Example::Markers2 />
                    <Card example=Example::Css />
                </div>
            </div>

            <section id="aspect-ratio" class="background-box">
                <h2 class="always-underline"><a href="examples.html#aspect-ratio">"Aspect ratio"</a></h2>
                <AspectRatioSunspots debug=app.debug.into() />
            </section>
        </article>
    }.into_any()
}
