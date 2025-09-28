use crate::pages::{
    demo::Demo,
    examples::{view_example, Example, Examples},
};
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    pub debug: RwSignal<bool>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_app_context();

    view! {
        <Router base="/leptos-chartistry">
            <SiteHeader />
            <main>
                <Routes fallback=NotFound>
                    <Route path=StaticSegment("") view=Demo />
                    <Route path=StaticSegment("examples.html") view=Examples />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::Line)) view=|| view_example(Example::Line) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::StackedLine)) view=|| view_example(Example::StackedLine) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::Bar)) view=|| view_example(Example::Bar) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::Legend)) view=|| view_example(Example::Legend) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::TickLabels)) view=|| view_example(Example::TickLabels) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::RotatedLabel)) view=|| view_example(Example::RotatedLabel) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::EdgeLayout)) view=|| view_example(Example::EdgeLayout) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::AxisMarker)) view=|| view_example(Example::AxisMarker) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::GridLine)) view=|| view_example(Example::GridLine) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::GuideLine)) view=|| view_example(Example::GuideLine) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::InsetLegend)) view=|| view_example(Example::InsetLegend) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::InnerLayout)) view=|| view_example(Example::InnerLayout) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::MixedInterpolation)) view=|| view_example(Example::MixedInterpolation) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::Stepped)) view=|| view_example(Example::Stepped) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::Tooltip)) view=|| view_example(Example::Tooltip) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::Colours)) view=|| view_example(Example::Colours) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::Markers)) view=|| view_example(Example::Markers) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::Markers2)) view=|| view_example(Example::Markers2) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::LineGradient)) view=|| view_example(Example::LineGradient) />
                    <Route path=(StaticSegment("examples"), StaticSegment(Example::Css)) view=|| view_example(Example::Css) />
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
            <div class="background-box">
                <h1 class="always-underline">"Page not found"</h1>
                <p>"The page you are looking for does not exist."</p>
            </div>
        </article>
    }
    .into_any()
}

pub fn provide_app_context() {
    provide_context(State::default());
}

pub fn use_app_context() -> State {
    use_context::<State>().unwrap()
}
