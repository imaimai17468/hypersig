# use axum::Router;
# use axum::http::{StatusCode, header};
# use axum::response::{IntoResponse, Response};
# use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
# use serde::{Deserialize, Serialize};
# use tower_sessions::SessionManagerLayer;
# #[derive(Signals, Serialize, Deserialize)]
# struct Home {
#     name: String,
# }
# #[derive(Signals, Serialize, Deserialize)]
# struct Login {
#     name: String,
# }
# #[derive(Clone)]
# struct AppState;
# fn checked(name: &str, _password: &str) -> bool {
#     name == "ada"
# }
# const MEMBER: &str = "member";
# #[derive(Debug)]
# struct StoreFailed(tower_sessions::session::Error);
# impl IntoResponse for StoreFailed {
#     fn into_response(self) -> Response {
#         (StatusCode::INTERNAL_SERVER_ERROR, "Try again.").into_response()
#     }
# }
# #[derive(Serialize, Deserialize)]
# struct Credentials {
#     name: String,
#     password: String,
# }
# const SIGN_IN: Action<Login, Credentials> = Action::post("/sign-in");
# async fn home(_: AppState, (): ()) -> Page<Home> {
#     Page::new("Home", Home { name: String::new() })
# }
# async fn login(_: AppState, (): ()) -> Page<Login> {
#     Page::new("Sign in", Login { name: String::new() })
# }
# async fn sign_in(
#     _: AppState,
#     session: Session,
#     credentials: Credentials,
# ) -> Result<Patches<Login>, StoreFailed> {
#     if !checked(&credentials.name, &credentials.password) {
#         return Ok(Patches::new());
#     }
#     session.cycle_id().await.map_err(StoreFailed)?;
#     session.insert(MEMBER, credentials.name).await.map_err(StoreFailed)?;
#     Ok(Patches::new().navigate(HOME.url(())))
# }
# hypersig::routes! {
#     fn pages(Router<AppState>);
#     const HOME: Route = "/" => home;
#     const LOGIN: Route = "/login" => login;
# }
# fn app() -> Router {
#     pages(Router::new())
#         .hypersig_runtime()
#         .action(SIGN_IN, sign_in)
#         .with_state(AppState)
#         .layer(SessionManagerLayer::new(MemoryStore::default()).with_name("session"))
# }
use std::sync::Arc;

use tower::ServiceExt as _;
use tower_sessions::{MemoryStore, Session};

# tokio::runtime::Runtime::new().unwrap().block_on(async {
// The body of a `#[tokio::test] async fn`. The test builds the `Session` the
// layer would hand the handler, with a store of its own.
let session = Session::new(None, Arc::new(MemoryStore::default()), None);
let credentials = Credentials { name: "ada".to_owned(), password: "secret".to_owned() };
let patches = sign_in(AppState, session.clone(), credentials).await.unwrap();
assert_eq!(patches.navigates_to(), Some(&HOME.url(())));
let member = session.get::<String>(MEMBER).await.unwrap();
assert_eq!(member.as_deref(), Some("ada"));

// Through the router, the layer reads and writes the cookie as it does for a
// visitor.
let credentials = Credentials { name: "ada".to_owned(), password: "secret".to_owned() };
let response = app().oneshot(SIGN_IN.request(&credentials)).await.unwrap();
assert_eq!(response.status(), StatusCode::OK);
let cookie = response.headers().get(header::SET_COOKIE).unwrap();
assert!(cookie.to_str().unwrap().starts_with("session="));
# });
