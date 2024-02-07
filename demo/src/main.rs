use demo::pages;
use leptos::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <SiteHeader />
        <main>
            <pages::demo::Demo />
        </main>
    }
}

#[component]
fn SiteHeader() -> impl IntoView {
    view! {
        <header>
            <h2><a href="https://github.com/feral-dot-io/leptos-chartistry">"leptos-chartistry"</a></h2>
            <nav>
                <p><a href="/">"Demo"</a></p>
                <p><a href="/examples">"Examples"</a></p>
            </nav>
            <div class="badges">
                <p>
                    <a href="https://github.com/feral-dot-io/leptos-chartistry">
                        <img src="https://img.shields.io/badge/github-blue?logo=github&style=for-the-badge" alt="GitHub" height="28px" />
                    </a>
                </p>
                <p>
                    <a href="https://crates.io/crates/leptos-chartistry">
                        <img src="https://img.shields.io/crates/v/leptos-chartistry.svg?style=for-the-badge" alt="Crates.io version" height="28px" />
                    </a>
                </p>
                <p>
                    <a href="https://docs.rs/leptos-chartistry">
                        <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=for-the-badge" alt="Docs.rs" height="28px" />
                    </a>
                </p>
            </div>
        </header>
    }
}
