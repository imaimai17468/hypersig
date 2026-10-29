# use axum::Router;
# use axum::http::StatusCode;
# use axum::response::{IntoResponse, Response};
# use hypersig::html::{a, button, form, input, p};
# use hypersig::{Action, Page, Patches, Route, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Clone)]
# struct AppState;
# impl AppState {
#     fn save(&self, _name: &str, _note: String) {}
# }
# const MEMBER: &str = "member";
# #[derive(Signals, Serialize, Deserialize)]
# struct Login {
#     problem: String,
# }
# async fn login(_: AppState, (): ()) -> Page<Login> {
#     Page::new("Sign in", Login { problem: String::new() })
# }
# #[derive(Debug)]
# struct StoreFailed(tower_sessions::session::Error);
# impl IntoResponse for StoreFailed {
#     fn into_response(self) -> Response {
#         (StatusCode::INTERNAL_SERVER_ERROR, "Try again.").into_response()
#     }
# }
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use hypersig::{PageResponse, RequestKind};
use tower_sessions::Session;

/// The member whose session cookie came with the request.
struct Member {
    name: String,
}

impl FromRequestParts<AppState> for Member {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Response> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(IntoResponse::into_response)?;
        match session.get::<String>(MEMBER).await {
            Ok(Some(name)) => Ok(Self { name }),
            // A page goes to the sign-in page. An action's `fetch` would
            // follow a redirect, so it gets a 401 its `Call::error` shows.
            Ok(None) => Err(match RequestKind::of(parts) {
                RequestKind::Navigation => PageResponse::redirect(LOGIN.url(())).into_response(),
                RequestKind::Runtime => (StatusCode::UNAUTHORIZED, "Sign in again.").into_response(),
            }),
            Err(error) => Err(StoreFailed(error).into_response()),
        }
    }
}

#[derive(Signals, Serialize, Deserialize)]
struct Home {
    name: String,
    draft: String,
    problem: String,
    /// The status of the last failed call, or `0`.
    failure: i32,
}

async fn home(_: AppState, member: Member, (): ()) -> Page<Home> {
    let signals = Home {
        name: member.name,
        draft: String::new(),
        problem: String::new(),
        failure: 0,
    };
    let signed_out = Home::failure().equals(i32::from(StatusCode::UNAUTHORIZED.as_u16()));
    Page::new("Home", signals).child(
        form()
            .on_submit(
                ADD_NOTE
                    .call_with(Home::draft())
                    .error(Home::problem())
                    .status(Home::failure()),
            )
            .child(input().bind(Home::draft()))
            .child(button().text("Add note"))
            .child(p().bind_text(Home::problem()))
            .child(a().show(signed_out).href(LOGIN.link()).text("Sign in again")),
    )
}

async fn add_note(state: AppState, member: Member, note: String) -> Patches<Home> {
    state.save(&member.name, note);
    Home::draft().patch(String::new()).into()
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const HOME: Route = "/" => home;
    const LOGIN: Route = "/login" => login;
    action ADD_NOTE: Action<Home, String> = post "/notes" => add_note;
}
# let _: Router<AppState> = pages(Router::new());
