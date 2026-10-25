use tower_service::Service as _;
use worker::{Context, Env, HttpRequest, event};

#[event(fetch)]
async fn fetch(
    request: HttpRequest,
    _env: Env,
    _context: Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    let mut app = pages(Router::new()).hypersig_runtime();
    Ok(app.call(request).await?)
}
