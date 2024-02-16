use demo::pages::{demo::Demo, examples::Examples};
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <SiteHeader />
            <main>
                <Routes base="/leptos-chartistry".to_string()>
                    <Route path="/" view=Demo />
                    <Route path="/examples.html" view=Examples />
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
            <h1>"Page not found"</h1>
            <p class="background-box">"The page you are looking for does not exist."</p>
        </article>
    }
}
