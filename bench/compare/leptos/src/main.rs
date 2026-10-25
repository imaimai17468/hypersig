//! The axum server for the Leptos app in `lib.rs`. `cargo leptos build`
//! compiles it with the `ssr` feature and the wasm with `hydrate`.

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    use axum::Router;
    use axum::http::StatusCode;
    use axum::routing::post;
    use bench_leptos::{App, shell, store};
    use leptos::prelude::get_configuration;
    use leptos_axum::{LeptosRoutes, generate_route_list};

    *store::lock() = store::seed();

    // `None` reads the LEPTOS_* environment variables, which the harness sets.
    let options = get_configuration(None)
        .expect("Leptos configuration")
        .leptos_options;
    let addr = options.site_addr;
    let routes = generate_route_list(App);
    let app = Router::new()
        .route(
            "/bench-reset",
            post(|| async {
                *store::lock() = store::seed();
                StatusCode::NO_CONTENT
            }),
        )
        .leptos_routes(&options, routes, {
            let options = options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(options);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await
}

#[cfg(not(feature = "ssr"))]
fn main() {}
