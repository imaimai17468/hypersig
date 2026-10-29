# use axum::response::{IntoResponse, Response};
use hypersig::{PageResponse, RequestKind};

fn signed_out(kind: RequestKind) -> Response {
    match kind {
        // `redirect` takes a `Url`, which only a route's `url` builds.
        RequestKind::Navigation => PageResponse::redirect("/login").into_response(),
        RequestKind::Runtime => "Sign in again.".into_response(),
    }
}
# let _ = signed_out;
