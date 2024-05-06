use crate::pages::{
    demo::Demo,
    examples::{Examples, *},
};
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    pub debug: RwSignal<bool>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_app_context();

    view! {
        <Router>
            <SiteHeader />
            <main>
                <Routes base="/leptos-chartistry".to_string()>
                    <Route path="/index" view=Demo />
                    <Route path="/examples.html" view=Examples />
                    <Route path="/examples/line-chart.html" view=LineExamplePage />
                    <Route path="/examples/line-chart.html" view=LineExamplePage />
                    <Route path="/examples/stacked-line-chart.html" view=StackedLineExamplePage />
                    <Route path="/examples/bar-chart.html" view=BarExamplePage />
                    <Route path="/examples/legend-html" view=LegendExamplePage />
                    <Route path="/examples/tick-labels.html" view=TickLabelsExamplePage />
                    <Route path="/examples/rotated-label.html" view=RotatedLabelExamplePage />
                    <Route path="/examples/combined-edge-layout.html" view=EdgeLayoutExamplePage />
                    <Route path="/examples/axis-marker.html" view=AxisMarkerExamplePage />
                    <Route path="/examples/grid-line.html" view=GridLineExamplePage />
                    <Route path="/examples/guide-line.html" view=GuideLineExamplePage />
                    <Route path="/examples/inset-legend.html" view=InsetLegendExamplePage />
                    <Route path="/examples/combined-inner-layout.html" view=InnerLayoutExamplePage />
                    <Route path="/examples/linear-and monotone.html" view=MixedInterpolationExamplePage />
                    <Route path="/examples/stepped.html" view=SteppedExamplePage />
                    <Route path="/examples/tooltip.html" view=TooltipExamplePage />
                    <Route path="/examples/colour.html" view=ColoursExamplePage />
                    <Route path="/examples/point-markers.html" view=MarkersExamplePage />
                    <Route path="/examples/point-markers-2.html" view=Markers2ExamplePage />
                    <Route path="/examples/line-colour-scheme.html" view=LineGradientExamplePage />
                    <Route path="/examples/css-styles.html" view=CssExamplePage />
                    <Route path="/*any" view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn SiteHeader() -> impl IntoView {
    view! {
        <header>
            <h2><a rel="external" href="https://github.com/feral-dot-io/leptos-chartistry">"leptos-chartistry"</a></h2>
            <nav>
                <p><A href="/leptos-chartistry/">"Demo"</A></p>
                <p><A href="/leptos-chartistry/examples.html">"Examples"</A></p>
            </nav>
            <div class="badges">
                <p>
                    <a rel="external" href="https://github.com/feral-dot-io/leptos-chartistry">
                        <img src="https://img.shields.io/badge/github-blue?logo=github&style=for-the-badge" alt="GitHub" height="28px" />
                    </a>
                </p>
                <p>
                    <a rel="external" href="https://crates.io/crates/leptos-chartistry">
                        <img src="https://img.shields.io/crates/v/leptos-chartistry.svg?style=for-the-badge" alt="Crates.io version" height="28px" />
                    </a>
                </p>
                <p>
                    <a rel="external" href="https://docs.rs/leptos-chartistry">
                        <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=for-the-badge" alt="Docs.rs" height="28px" />
                    </a>
                </p>
            </div>
        </header>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <article id="status404">
            <p class="background-box">
                <h1 class="connect-heading">"Page not found"</h1>
                "The page you are looking for does not exist."
            </p>
        </article>
    }
}

pub fn provide_app_context() {
    provide_context(State::default());
}

pub fn use_app_context() -> State {
    use_context::<State>().unwrap()
}
