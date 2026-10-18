use axum::Router;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderName, StatusCode, header};
use axum::response::{IntoResponse, Redirect, Response};
use hypersig::{Action, Patches, RouterExt, Signals};
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
        // An action's `fetch` asks for `text/event-stream` and would follow a redirect.
        let accept = parts.headers.get(header::ACCEPT);
        let from_action = accept.is_some_and(|accept| accept == "text/event-stream");
        name.map(Member).ok_or(SignedOut { from_action })
    }
}

struct SignedOut {
    from_action: bool,
}

impl IntoResponse for SignedOut {
    fn into_response(self) -> Response {
        if self.from_action {
            (StatusCode::UNAUTHORIZED, "Sign in again.").into_response()
        } else {
            Redirect::to("/login").into_response()
        }
    }
}

const REFRESH: Action<Home> = Action::post("/refresh");
const SIGN_OUT: Action<Home> = Action::post("/sign-out");

// The extractor sits between the state and the arguments.
async fn refresh(_: AppState, Member(name): Member, (): ()) -> Patches<Home> {
    Home::name().patch(name).into()
}

// Headers go beside the patches.
type Cookie = [(HeaderName, &'static str); 1];

async fn sign_out(_: AppState, _: Member, (): ()) -> (Cookie, Patches<Home>) {
    let forget = [(header::SET_COOKIE, "session=; Max-Age=0; HttpOnly; Path=/")];
    (forget, Patches::new())
}

let _app: Router = Router::new()
    .action(REFRESH, refresh)
    .action(SIGN_OUT, sign_out)
    .with_state(AppState);
