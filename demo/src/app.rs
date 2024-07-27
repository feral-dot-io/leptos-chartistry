use crate::pages::{
    demo::Demo,
    examples::{self, Examples},
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
        <Router>
            <SiteHeader />
            <main>
                <Routes fallback=NotFound>
                    <Route path=StaticSegment("/leptos-chartistry/") view=Demo />
                    <Route path=StaticSegment("/leptos-chartistry/examples.html") view=Examples />
                    <examples::Routes />
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
