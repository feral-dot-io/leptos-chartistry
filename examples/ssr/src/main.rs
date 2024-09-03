#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use my_example_ssr::{app::App, fallback::file_and_error_handler};

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    
    let routes = generate_route_list(App);
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, || view! { <App/> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
}
