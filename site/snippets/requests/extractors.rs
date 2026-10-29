use axum::Router;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use hypersig::{Action, Page, PageResponse, Patches, RequestKind, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Home {
    name: String,
}

#[derive(Clone)]
struct AppState;

impl AppState {
    fn member_of(&self, token: &str) -> Option<String> {
        (token == "abc").then(|| "Ada".to_owned())
    }
}

/// The member whose session cookie came with the request.
struct Member(String);

impl FromRequestParts<AppState> for Member {
    type Rejection = SignedOut;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, SignedOut> {
        let cookies = parts.headers.get(header::COOKIE).and_then(|value| value.to_str().ok());
        let token = cookies.and_then(|cookies| {
            cookies.split(';').find_map(|cookie| cookie.trim().strip_prefix("session="))
        });
        let name = token.and_then(|token| state.member_of(token));
        name.map(Member).ok_or(SignedOut(RequestKind::of(parts)))
    }
}

struct SignedOut(RequestKind);

impl IntoResponse for SignedOut {
    fn into_response(self) -> Response {
        match self.0 {
            // An action's `fetch` would follow a redirect, so it gets a status.
            RequestKind::Runtime => (StatusCode::UNAUTHORIZED, "Sign in again.").into_response(),
            RequestKind::Navigation => PageResponse::redirect(LOGIN.url(())).into_response(),
        }
    }
}

# async fn login(_: AppState, (): ()) -> Page<Home> {
#     Page::new("Sign in", Home { name: String::new() })
# }
hypersig::routes! {
    fn pages(Router<AppState>);
    const LOGIN: Route = "/login" => login;
    action REFRESH: Action<Home> = post "/refresh" => refresh;
}

// The extractor sits between the state and the arguments.
async fn refresh(_: AppState, Member(name): Member, (): ()) -> Patches<Home> {
    Home::name().patch(name).into()
}

let _app: Router = pages(Router::new()).with_state(AppState);
