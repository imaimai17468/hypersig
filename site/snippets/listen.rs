# use axum::Router;
# async fn serve(app: Router) -> std::io::Result<()> {
// A container host names the port in `PORT` and connects from outside the
// container, so listen on every interface.
let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_owned());
let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
// The server stops once the requests in progress finish. A stream that
// stays open holds it until the host's grace period ends.
axum::serve(listener, app)
    .with_graceful_shutdown(shutdown())
    .await
# }

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
