//! The check every request passes before the application's router sees it.
//!
//! The server listens on `127.0.0.1`, where any program on the machine can
//! reach it, and hypersig's own checks stop only a request a browser sends
//! from another site. This one stops the rest:
//!
//! - a request whose `Host` is not `127.0.0.1:<port>`, which is how a page
//!   whose domain was made to resolve to `127.0.0.1` arrives;
//! - a request without the session cookie, which only the application's
//!   first window receives, by loading its first page with the start token
//!   in a header, once.
//!
//! It answers one request itself: a second copy's launch, posted to
//! [`handoff::PATH`] with the handoff token, which this run wrote next to
//! its lock.

use std::fmt::Write as _;
use std::io;
use std::sync::{Arc, Mutex, PoisonError};

use axum::Router;
use axum::extract::{Request, State};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use tokio::sync::broadcast;

use crate::handoff::{self, Launch};

/// The header the first window's first request carries the start token in,
/// which the answer trades for the session cookie.
pub(crate) const START_HEADER: &str = "hypersig-desktop-start";

/// The name of the session cookie.
const COOKIE: &str = "hypersig-desktop";

/// The tokens one run of the application accepts.
#[derive(Debug)]
pub(crate) struct Guard {
    host: HeaderValue,
    start: Mutex<Option<Secret>>,
    session: Secret,
    handoff: Secret,
    launches: broadcast::Sender<Launch>,
}

/// The tokens a new [`Guard`] gives out: the start token for the first
/// window's first request, and the handoff token for a second copy.
pub(crate) struct Tokens {
    pub(crate) start: HeaderValue,
    pub(crate) handoff: String,
}

impl Guard {
    /// The guard of a server on `port`, which sends each launch a second
    /// copy hands over to `launches`, and the tokens it accepts.
    ///
    /// # Errors
    ///
    /// Fails when the system gives no random bytes.
    pub(crate) fn new(
        port: u16,
        launches: broadcast::Sender<Launch>,
    ) -> io::Result<(Arc<Self>, Tokens)> {
        let start = Secret::new()?;
        let handoff = Secret::new()?;
        let tokens = Tokens {
            start: HeaderValue::try_from(start.hex()).expect("hex digits are header text"),
            handoff: handoff.hex(),
        };
        let guard = Self {
            host: HeaderValue::try_from(format!("127.0.0.1:{port}"))
                .expect("an address and a port are header text"),
            start: Mutex::new(Some(start)),
            session: Secret::new()?,
            handoff,
            launches,
        };
        Ok((Arc::new(guard), tokens))
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
    if request.method() == Method::POST
        && request.uri().path() == handoff::PATH
        && request
            .headers()
            .get(handoff::HEADER)
            .and_then(|offered| offered.to_str().ok())
            .is_some_and(|offered| guard.handoff.matches(offered))
    {
        return guard.take_launch(request).await;
    }
    // A reload may send the first request's headers again, after the token
    // is spent, so a token that does not match leaves the cookie to decide.
    if let Some(offered) = request.headers().get(START_HEADER)
        && guard.take_start(offered)
    {
        let mut answer = next.run(request).await;
        let cookie = format!(
            "{COOKIE}={}; HttpOnly; SameSite=Strict; Path=/",
            guard.session.hex()
        );
        answer.headers_mut().append(
            header::SET_COOKIE,
            HeaderValue::try_from(cookie).expect("hex digits are header text"),
        );
        return answer;
    }
    if carries_session(&guard.session, request.headers()) {
        next.run(request).await
    } else {
        StatusCode::FORBIDDEN.into_response()
    }
}

impl Guard {
    /// Reads the launch a second copy posted, and sends it on.
    async fn take_launch(&self, request: Request) -> Response {
        let Ok(body) = axum::body::to_bytes(request.into_body(), handoff::LIMIT).await else {
            return StatusCode::PAYLOAD_TOO_LARGE.into_response();
        };
        let Some(launch) = Launch::decode(&body) else {
            return StatusCode::BAD_REQUEST.into_response();
        };
        // With no `Launches` taken, the launch only brings a window forward.
        let _ = self.launches.send(launch);
        StatusCode::NO_CONTENT.into_response()
    }

    /// Whether `offered` is the start token, which works once.
    fn take_start(&self, offered: &HeaderValue) -> bool {
        let mut start = self.start.lock().unwrap_or_else(PoisonError::into_inner);
        let accepted = start
            .as_ref()
            .is_some_and(|token| offered.to_str().is_ok_and(|offered| token.matches(offered)));
        if accepted {
            *start = None;
        }
        accepted
    }
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
    use axum::http::{HeaderValue, Request, StatusCode, header};
    use axum::routing::get;
    use tower::ServiceExt as _;

    use super::{Guard, START_HEADER, wrap};

    const PORT: u16 = 4321;
    const HOST: &str = "127.0.0.1:4321";

    fn app() -> (Router, HeaderValue) {
        let (guard, tokens) = Guard::new(PORT, crate::handoff::channel()).unwrap();
        let start = tokens.start;
        let router = Router::new().route("/home", get(|| async { "home" }));
        (wrap(router, guard), start)
    }

    fn request(host: &str, cookie: Option<&str>, start: Option<&HeaderValue>) -> Request<Body> {
        let mut request = Request::get("/home").header(header::HOST, host);
        if let Some(cookie) = cookie {
            request = request.header(header::COOKIE, cookie);
        }
        if let Some(start) = start {
            request = request.header(START_HEADER, start);
        }
        request.body(Body::empty()).unwrap()
    }

    /// Loads the first page with the start token and returns the
    /// `name=value` of the cookie its answer sets.
    async fn cookie(app: &Router, start: &HeaderValue) -> String {
        let answer = app
            .clone()
            .oneshot(request(HOST, None, Some(start)))
            .await
            .unwrap();
        assert_eq!(answer.status(), StatusCode::OK);
        let set = answer.headers()[header::SET_COOKIE].to_str().unwrap();
        set.split(';').next().unwrap().to_owned()
    }

    #[tokio::test]
    async fn the_window_with_its_cookie_is_answered() {
        let (app, start) = app();
        let cookie = cookie(&app, &start).await;
        let answer = app
            .oneshot(request(HOST, Some(&cookie), None))
            .await
            .unwrap();
        assert_eq!(answer.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn a_request_without_the_cookie_is_refused_even_where_no_route_matches() {
        let (app, _) = app();
        let nowhere = Request::get("/nothing-here")
            .header(header::HOST, HOST)
            .body(Body::empty())
            .unwrap();
        assert_eq!(
            app.clone().oneshot(nowhere).await.unwrap().status(),
            StatusCode::FORBIDDEN
        );
        let answer = app
            .clone()
            .oneshot(request(HOST, None, None))
            .await
            .unwrap();
        assert_eq!(answer.status(), StatusCode::FORBIDDEN);
        let answer = app
            .oneshot(request(HOST, Some("hypersig-desktop=00"), None))
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
                .oneshot(request(host, Some(&cookie), None))
                .await
                .unwrap();
            assert_eq!(answer.status(), StatusCode::FORBIDDEN, "{host}");
        }
    }

    #[tokio::test]
    async fn only_the_start_token_works_and_only_once() {
        let (app, start) = app();
        let guessed = HeaderValue::from_static("00");
        let answer = app
            .clone()
            .oneshot(request(HOST, None, Some(&guessed)))
            .await
            .unwrap();
        assert_eq!(answer.status(), StatusCode::FORBIDDEN);

        let cookie = cookie(&app, &start).await;
        let again = app
            .clone()
            .oneshot(request(HOST, None, Some(&start)))
            .await
            .unwrap();
        assert_eq!(again.status(), StatusCode::FORBIDDEN);

        // A reload that sends the spent token again, with the cookie.
        let reload = app
            .oneshot(request(HOST, Some(&cookie), Some(&start)))
            .await
            .unwrap();
        assert_eq!(reload.status(), StatusCode::OK);
        assert!(!reload.headers().contains_key(header::SET_COOKIE));
    }

    #[tokio::test]
    async fn a_launch_with_the_handoff_token_is_taken_and_one_without_is_refused() {
        use std::ffi::OsString;
        use std::path::PathBuf;

        use crate::handoff::{self, Launch};

        let launches = handoff::channel();
        let mut heard = launches.subscribe();
        let (guard, tokens) = Guard::new(PORT, launches).unwrap();
        let app = wrap(Router::new(), guard);
        let launch = Launch::new(PathBuf::from("/home/me"), vec![OsString::from("today.md")]);
        let post = |token: &str| {
            Request::post(handoff::PATH)
                .header(header::HOST, HOST)
                .header(handoff::HEADER, token)
                .body(Body::from(launch.encode()))
                .unwrap()
        };

        let answer = app.clone().oneshot(post(&"0".repeat(64))).await.unwrap();
        assert_eq!(answer.status(), StatusCode::FORBIDDEN);
        heard.try_recv().unwrap_err();

        let answer = app.oneshot(post(&tokens.handoff)).await.unwrap();
        assert_eq!(answer.status(), StatusCode::NO_CONTENT);
        assert_eq!(heard.try_recv().unwrap(), launch);
    }
}
