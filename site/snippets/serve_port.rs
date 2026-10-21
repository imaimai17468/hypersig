# use axum::Router;
# use hypersig::RouterExt;
# #[tokio::main]
# async fn main() -> std::io::Result<()> {
# let app: Router = Router::new().hypersig_runtime();
// The host names the port in `PORT` and connects from outside the
// container, so listen on every interface.
let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_owned());
let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
axum::serve(listener, app).await
# }
