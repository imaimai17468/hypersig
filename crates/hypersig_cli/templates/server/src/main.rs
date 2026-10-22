//! A counter whose increments happen in the browser and whose reset goes to the server.
//!
//! Run it with `cargo run` and open <http://localhost:3000>.
//! `cargo hypersig dev` runs it too, and starts it again after each change.
//! Build the container image with `docker build -t {{project-name}} .` and
//! run it with `docker run --rm -p 3000:3000 {{project-name}}`.

use axum::Router;
use hypersig::html::{button, div, span};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
}

const RESET: Action<Counter> = Action::post("/reset");

async fn index((): (), (): ()) -> Page<Counter> {
    let count = Counter::count();
    Page::new("Counter", Counter { count: 0 }).child(
        div()
            .child(button().on_click(count.set(count + 1)).text("+1"))
            .child(span().bind_text(count.to_text()))
            .child(button().on_click(RESET.call()).text("Reset on the server")),
    )
}

async fn reset((): (), (): ()) -> Patches<Counter> {
    Counter::count().patch(0).into()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = pages(Router::new()).hypersig_runtime().action(RESET, reset);
    // A container host names the port in `PORT` and connects from outside the
    // container, so listen on every interface.
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_owned());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    // The server stops once the requests in progress finish. A stream that
    // stays open holds it until the host's grace period ends.
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await
}

/// Waits for Ctrl+C, or for the SIGTERM a container host sends to stop the
/// container. The server runs as the container's first process, which the
/// kernel does not stop on a signal it does not handle.
async fn shutdown() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        let mut terminate = signal(SignalKind::terminate()).expect("SIGTERM can be watched");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = terminate.recv() => {}
        }
    }
    #[cfg(not(unix))]
    let _ = tokio::signal::ctrl_c().await;
}
