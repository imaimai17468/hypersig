//! The check every request passes before the application's router sees it.
//!
//! The server listens on `127.0.0.1`, where any program on the machine can
//! reach it, and hypersig's own checks stop only a request a browser sends
//! from another site. This one stops the rest:
//!
//! - a request whose `Host` is not `127.0.0.1:<port>`, which is how a page
//!   whose domain was made to resolve to `127.0.0.1` arrives;
//! - a request without the session cookie, which only the application's
//!   window receives, by opening the start URL with the start token once.

use std::fmt::Write as _;
use std::io;
use std::sync::{Arc, Mutex, PoisonError};

use axum::Router;
use axum::extract::{Request, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};

/// The path the first window opens, which trades the start token for the session cookie.
const START_PATH: &str = "/.hypersig-desktop/start";

/// The name of the session cookie.
const COOKIE: &str = "hypersig-desktop";

/// The tokens one run of the application accepts.
#[derive(Debug)]
pub(crate) struct Guard {
    host: HeaderValue,
    start: Mutex<Option<Secret>>,
    session: Secret,
    first_page: HeaderValue,
}

impl Guard {
    /// The guard of a server on `port`, which sends the window to
    /// `first_page` once it has its cookie, and the URL the window opens to
    /// receive it.
    ///
    /// # Errors
    ///
    /// Fails when the system gives no random bytes.
    pub(crate) fn new(port: u16, first_page: &str) -> io::Result<(Arc<Self>, String)> {
        let start = Secret::new()?;
        let url = format!("http://127.0.0.1:{port}{START_PATH}?token={}", start.hex());
        let guard = Self {
            host: HeaderValue::try_from(format!("127.0.0.1:{port}"))
                .expect("an address and a port are header text"),
            start: Mutex::new(Some(start)),
            session: Secret::new()?,
            first_page: HeaderValue::try_from(first_page).map_err(io::Error::other)?,
        };
        Ok((Arc::new(guard), url))
    }
}

/// `router` behind `guard`, which sees every request, including the ones no
/// route of `router` matches.
pub(crate) fn wrap(router: Router, guard: Arc<Guard>) -> Router {
    Router::new()
        .fallback_service(router)
        .layer(middleware::from_fn_with_state(guard, check))
}

async fn check(State(guard): State<Arc<Guard>>, request: Request, next: Next) -> Response {
    if request.headers().get(header::HOST) != Some(&guard.host) {
        return StatusCode::FORBIDDEN.into_response();
    }
    if request.uri().path() == START_PATH {
        return start(&guard, request.uri().query());
    }
    if carries_session(&guard.session, request.headers()) {
        next.run(request).await
    } else {
        StatusCode::FORBIDDEN.into_response()
    }
}

/// Answers the start URL: the session cookie and a redirect to the first
/// page, once, for the query that carries the start token.
fn start(guard: &Guard, query: Option<&str>) -> Response {
    let offered = query.and_then(|query| query.strip_prefix("token="));
    let mut start = guard.start.lock().unwrap_or_else(PoisonError::into_inner);
    let accepted = start
        .as_ref()
        .is_some_and(|token| offered.is_some_and(|offered| token.matches(offered)));
    if !accepted {
        return StatusCode::FORBIDDEN.into_response();
    }
    *start = None;
    let cookie = format!(
        "{COOKIE}={}; HttpOnly; SameSite=Strict; Path=/",
        guard.session.hex()
    );
    (
        StatusCode::SEE_OTHER,
        [
            (header::LOCATION, guard.first_page.clone()),
            (
                header::SET_COOKIE,
                HeaderValue::try_from(cookie).expect("hex digits are header text"),
            ),
        ],
    )
        .into_response()
}

/// Whether one of the `Cookie` headers carries `session`.
fn carries_session(session: &Secret, headers: &HeaderMap) -> bool {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .filter_map(|pair| pair.trim().strip_prefix(COOKIE)?.strip_prefix('='))
        .any(|value| session.matches(value))
}

/// 32 random bytes.
#[derive(Debug)]
struct Secret([u8; 32]);

impl Secret {
    fn new() -> io::Result<Self> {
        let mut bytes = [0; 32];
        getrandom::fill(&mut bytes).map_err(io::Error::other)?;
        Ok(Self(bytes))
    }

    fn hex(&self) -> String {
        self.0
            .iter()
            .fold(String::with_capacity(64), |mut text, byte| {
                write!(text, "{byte:02x}").expect("writing to a String does not fail");
                text
            })
    }

    /// Whether `text` is this secret in hex, in a time that does not depend
    /// on where the two first differ.
    fn matches(&self, text: &str) -> bool {
        let expected = self.hex();
        expected.len() == text.len()
            && expected
                .bytes()
                .zip(text.bytes())
                .fold(0, |differ, (a, b)| differ | (a ^ b))
                == 0
    }
}

#[cfg(test)]
mod tests {
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode, header};
    use axum::routing::get;
    use tower::ServiceExt as _;

    use super::{Guard, START_PATH, wrap};

    const PORT: u16 = 4321;

    fn app() -> (Router, String) {
        let (guard, start) = Guard::new(PORT, "/home").unwrap();
        let router = Router::new().route("/home", get(|| async { "home" }));
        (wrap(router, guard), start)
    }

    fn request(uri: &str, host: &str, cookie: Option<&str>) -> Request<Body> {
        let mut request = Request::get(uri).header(header::HOST, host);
        if let Some(cookie) = cookie {
            request = request.header(header::COOKIE, cookie);
        }
        request.body(Body::empty()).unwrap()
    }

    /// Opens the start URL and returns the `name=value` of the cookie it sets.
    async fn cookie(app: &Router, start: &str) -> String {
        let path = start.strip_prefix("http://127.0.0.1:4321").unwrap();
        let answer = app
            .clone()
            .oneshot(request(path, "127.0.0.1:4321", None))
            .await
            .unwrap();
        assert_eq!(answer.status(), StatusCode::SEE_OTHER);
        assert_eq!(answer.headers()[header::LOCATION], "/home");
        let set = answer.headers()[header::SET_COOKIE].to_str().unwrap();
        set.split(';').next().unwrap().to_owned()
    }

    #[tokio::test]
    async fn the_window_with_its_cookie_is_answered() {
        let (app, start) = app();
        let cookie = cookie(&app, &start).await;
        let answer = app
            .oneshot(request("/home", "127.0.0.1:4321", Some(&cookie)))
            .await
            .unwrap();
        assert_eq!(answer.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn a_request_without_the_cookie_is_refused_even_where_no_route_matches() {
        let (app, _) = app();
        for path in ["/home", "/nothing-here"] {
            let answer = app
                .clone()
                .oneshot(request(path, "127.0.0.1:4321", None))
                .await
                .unwrap();
            assert_eq!(answer.status(), StatusCode::FORBIDDEN, "{path}");
        }
        let answer = app
            .oneshot(request(
                "/home",
                "127.0.0.1:4321",
                Some("hypersig-desktop=00"),
            ))
            .await
            .unwrap();
        assert_eq!(answer.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn another_host_is_refused_with_the_cookie() {
        let (app, start) = app();
        let cookie = cookie(&app, &start).await;
        for host in ["evil.example:4321", "localhost:4321", "127.0.0.1:1234"] {
            let answer = app
                .clone()
                .oneshot(request("/home", host, Some(&cookie)))
                .await
                .unwrap();
            assert_eq!(answer.status(), StatusCode::FORBIDDEN, "{host}");
        }
    }

    #[tokio::test]
    async fn only_the_start_token_works_and_only_once() {
        let (app, start) = app();
        let guessed = app
            .clone()
            .oneshot(request(
                &format!("{START_PATH}?token=00"),
                "127.0.0.1:4321",
                None,
            ))
            .await
            .unwrap();
        assert_eq!(guessed.status(), StatusCode::FORBIDDEN);

        cookie(&app, &start).await;
        let path = start.strip_prefix("http://127.0.0.1:4321").unwrap();
        let again = app
            .oneshot(request(path, "127.0.0.1:4321", None))
            .await
            .unwrap();
        assert_eq!(again.status(), StatusCode::FORBIDDEN);
    }
}
