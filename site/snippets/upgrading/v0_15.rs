# use axum::Router;
# use axum::http::StatusCode;
# use axum::response::{IntoResponse, Response};
# use hypersig::{Page, PageResponse, RequestKind, Route, Signals};
# use serde::{Deserialize, Serialize};
#
# #[derive(Signals, Serialize, Deserialize)]
# struct Login {}
#
# async fn login((): (), (): ()) -> Page<Login> {
#     Page::new("Sign in", Login {})
# }
#
# hypersig::routes! {
#     fn pages(Router<()>);
#     const LOGIN: Route = "/login" => login;
# }
#
fn signed_out(kind: RequestKind) -> Response {
    match kind {
        RequestKind::Navigation => PageResponse::redirect(LOGIN.url(())).into_response(),
//-        RequestKind::Runtime => (StatusCode::UNAUTHORIZED, "Sign in again.").into_response(),
        _ => (StatusCode::UNAUTHORIZED, "Sign in again.").into_response(), //+
    }
}
# let _ = (signed_out, pages);
