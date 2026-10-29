# use axum::Router;
# use axum::http::StatusCode;
# use axum::response::{IntoResponse, Response};
# use hypersig::html::{attr, button, form, input, p};
# use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
# use serde::{Deserialize, Serialize};
use tower_sessions::cookie::SameSite;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};
# #[derive(Signals, Serialize, Deserialize)]
# struct Home {
#     name: String,
# }
# #[derive(Clone)]
# struct AppState;
# fn checked(name: &str, _password: &str) -> bool {
#     name == "ada"
# }
# async fn home(_: AppState, (): ()) -> Page<Home> {
#     Page::new("Home", Home { name: String::new() })
# }

/// The key under which a session holds the signed-in member's name.
const MEMBER: &str = "member";

#[derive(Signals, Serialize, Deserialize)]
struct Credentials {
    name: String,
    password: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Login {
    credentials: Credentials,
    problem: String,
}

#[derive(Debug)]
struct StoreFailed(tower_sessions::session::Error);

impl IntoResponse for StoreFailed {
    fn into_response(self) -> Response {
        eprintln!("session store: {}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, "Try again.").into_response()
    }
}

async fn login(_: AppState, (): ()) -> Page<Login> {
    let credentials = Login::credentials();
    let signals = Login {
        credentials: Credentials { name: String::new(), password: String::new() },
        problem: String::new(),
    };
    Page::new("Sign in", signals).child(
        form()
            .on_submit(SIGN_IN.call_with(credentials).error(Login::problem()))
            .child(input().bind(credentials.at(Credentials::name())))
            .child(
                input()
                    .attr(attr::TYPE, "password")
                    .bind(credentials.at(Credentials::password())),
            )
            .child(button().text("Sign in"))
            .child(p().bind_text(Login::problem())),
    )
}

async fn sign_in(
    _: AppState,
    session: Session,
    credentials: Credentials,
) -> Result<Patches<Login>, StoreFailed> {
    if !checked(&credentials.name, &credentials.password) {
        let wrong = "The name or the password is wrong.".to_owned();
        return Ok(Login::problem().patch(wrong).into());
    }
    session.cycle_id().await.map_err(StoreFailed)?;
    session.insert(MEMBER, credentials.name).await.map_err(StoreFailed)?;
    Ok(Patches::new().navigate(HOME.url(())))
}

async fn sign_out(_: AppState, session: Session, (): ()) -> Result<Patches<Home>, StoreFailed> {
    session.flush().await.map_err(StoreFailed)?;
    Ok(Patches::new().navigate(LOGIN.url(())))
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const HOME: Route = "/" => home;
    const LOGIN: Route = "/login" => login;
    action SIGN_IN: Action<Login, Credentials> = post "/sign-in" => sign_in;
    action SIGN_OUT: Action<Home> = post "/sign-out" => sign_out;
}

let sessions = SessionManagerLayer::new(MemoryStore::default())
    .with_name("session")
    .with_same_site(SameSite::Lax);
let _app: Router = pages(Router::new())
    .hypersig_runtime()
    .with_state(AppState)
    .layer(sessions);
