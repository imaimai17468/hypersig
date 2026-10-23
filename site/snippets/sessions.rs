# use axum::Router;
# use axum::extract::FromRequestParts;
# use axum::http::StatusCode;
# use axum::http::request::Parts;
# use axum::response::{IntoResponse, Response};
# use hypersig::{Action, Page, PageResponse, Patches, RequestKind, Route, RouterExt, Signals};
# use serde::{Deserialize, Serialize};
use tower_sessions::cookie::SameSite;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};
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
# async fn login(_: AppState, (): ()) -> Page<Login> {
#     Page::new("Sign in", Login { name: String::new() })
# }

/// The key under which a session holds the signed-in member's name.
const MEMBER: &str = "member";

/// The member whose session cookie came with the request.
struct Member {
    name: String,
    session: Session,
}

impl FromRequestParts<AppState> for Member {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Response> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(IntoResponse::into_response)?;
        match session.get::<String>(MEMBER).await {
            Ok(Some(name)) => Ok(Self { name, session }),
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

struct StoreFailed(tower_sessions::session::Error);

impl IntoResponse for StoreFailed {
    fn into_response(self) -> Response {
        eprintln!("session store: {}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, "Try again.").into_response()
    }
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    name: String,
    password: String,
}

const SIGN_IN: Action<Login, Credentials> = Action::post("/sign-in");
const SIGN_OUT: Action<Home> = Action::post("/sign-out");

async fn home(_: AppState, member: Member, (): ()) -> Page<Home> {
    Page::new("Home", Home { name: member.name })
}

async fn sign_in(
    _: AppState,
    session: Session,
    credentials: Credentials,
) -> Result<Patches<Login>, StoreFailed> {
    if !checked(&credentials.name, &credentials.password) {
        return Ok(Patches::new());
    }
    session.cycle_id().await.map_err(StoreFailed)?;
    session.insert(MEMBER, credentials.name).await.map_err(StoreFailed)?;
    Ok(Patches::new().navigate(HOME.url(())))
}

async fn sign_out(_: AppState, member: Member, (): ()) -> Result<Patches<Home>, StoreFailed> {
    member.session.flush().await.map_err(StoreFailed)?;
    Ok(Patches::new().navigate(LOGIN.url(())))
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const HOME: Route = "/" => home;
    const LOGIN: Route = "/login" => login;
}

let sessions = SessionManagerLayer::new(MemoryStore::default())
    .with_name("session")
    .with_same_site(SameSite::Lax);
let _app: Router = pages(Router::new())
    .hypersig_runtime()
    .action(SIGN_IN, sign_in)
    .action(SIGN_OUT, sign_out)
    .with_state(AppState)
    .layer(sessions);
