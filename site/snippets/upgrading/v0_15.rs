# use axum::http::StatusCode;
# use axum::response::{IntoResponse, Response};
# use hypersig::RequestKind;
#
fn signed_out(kind: RequestKind) -> Response {
    match kind {
        RequestKind::Navigation => (StatusCode::SEE_OTHER, "Sign in.").into_response(),
//-        RequestKind::Runtime => (StatusCode::UNAUTHORIZED, "Sign in again.").into_response(),
        _ => (StatusCode::UNAUTHORIZED, "Sign in again.").into_response(), //+
    }
}
# let _ = signed_out;
