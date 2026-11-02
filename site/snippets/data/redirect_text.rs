# use axum::response::{IntoResponse, Response};
use hypersig::{PageResponse, RequestKind};

fn signed_out(kind: RequestKind) -> Response {
    if kind == RequestKind::Navigation {
        // `redirect` takes a `Url`, which only a route's `url` builds.
        PageResponse::redirect("/login").into_response()
    } else {
        "Sign in again.".into_response()
    }
}
# let _ = signed_out;
