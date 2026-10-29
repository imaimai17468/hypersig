use axum::extract::Request;
use axum::http::{HeaderValue, header};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::{Extension, Router};
use hypersig::html::p;
use hypersig::{Framing, Origin, Page, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Report {
    total: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    const REPORT: Route = "/report" => report;
}

/// The nonce of one response.
#[derive(Clone)]
struct Nonce(String);

const PORTAL: Origin = Origin::new("https://portal.example");

async fn report((): (), Extension(nonce): Extension<Nonce>, (): ()) -> Page<Report> {
    Page::new("Report", Report { total: 12 })
        .csp_nonce(nonce.0)
        .framing(Framing::Origins(vec![PORTAL]))
        .child(p().bind_text(Report::total().to_text()))
}

/// Gives each request 128 random bits as its nonce, and answers with a
/// policy that allows a style only with that nonce and a script only from
/// this origin.
async fn content_security_policy(mut request: Request, next: Next) -> Response {
    let mut bytes = [0_u8; 16];
    getrandom::fill(&mut bytes).expect("the operating system provides random bytes");
    let nonce = format!("{:032x}", u128::from_ne_bytes(bytes));
    let policy = format!("default-src 'self'; script-src 'self'; style-src 'nonce-{nonce}'");
    request.extensions_mut().insert(Nonce(nonce));
    let mut response = next.run(request).await;
    // `append`, because `insert` would drop the page's `frame-ancestors`.
    response.headers_mut().append(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_str(&policy).expect("the policy is ASCII"),
    );
    response
}

fn app() -> Router {
    pages(Router::new())
        .hypersig_runtime()
        .layer(middleware::from_fn(content_security_policy))
}

# use tower::ServiceExt as _;
# tokio::runtime::Runtime::new().unwrap().block_on(async {
let request = Request::get(REPORT.url(()).as_str()).body(axum::body::Body::empty()).unwrap();
let response = app().oneshot(request).await.unwrap();
let policies: Vec<&str> = response
    .headers()
    .get_all(header::CONTENT_SECURITY_POLICY)
    .iter()
    .map(|value| value.to_str().unwrap())
    .collect();
assert_eq!(policies[0], "frame-ancestors 'self' https://portal.example");
assert!(policies[1].starts_with("default-src 'self'; script-src 'self'; style-src 'nonce-"));
# });
