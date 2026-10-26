//! The caching, sniffing, and buffering headers of the responses hypersig writes.

use axum::Router;
use axum::body::Body;
use axum::http::{HeaderMap, Request, header};
use futures::stream;
use hypersig::{Action, Page, Patches, Route, RouterExt as _, Signals, StreamAction};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

hypersig::routes! {
    fn app(Router<()>);
    const HOME: Route = "/" => |(), ()| async { Page::new("Home", Counter { count: 0 }) };
    const PUBLIC: Route = "/public" => |(), ()| async {
        (
            [(header::CACHE_CONTROL, "public, max-age=60")],
            Page::new("Public", Counter { count: 0 }),
        )
    };
    action ADD: Action<Counter, i32> = post "/add" => |(), count: i32| async move {
        Patches::from(Counter::count().patch(count + 1))
    };
    stream TICKS: StreamAction<Counter, i32> = "/ticks" => |(), from: i32| {
        stream::iter([Patches::from(Counter::count().patch(from))])
    };
}

async fn headers(request: Request<Body>) -> HeaderMap {
    app(Router::new())
        .hypersig_runtime()
        .oneshot(request)
        .await
        .expect("the router is infallible")
        .headers()
        .clone()
}

fn get(path: &str) -> Request<Body> {
    Request::get(path).body(Body::empty()).expect("request")
}

fn value<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers
        .get(name)
        .map(|value| value.to_str().expect("ASCII"))
}

#[tokio::test]
async fn a_page_is_kept_by_no_shared_cache_and_revalidated_by_the_browser() {
    let headers = headers(get("/")).await;
    assert_eq!(value(&headers, "cache-control"), Some("private, no-cache"));
    assert_eq!(value(&headers, "x-content-type-options"), Some("nosniff"));
}

#[tokio::test]
async fn a_cache_control_the_handler_sends_replaces_the_page_s() {
    let headers = headers(get("/public")).await;
    assert_eq!(value(&headers, "cache-control"), Some("public, max-age=60"));
}

#[tokio::test]
async fn an_action_and_its_refusal_are_not_sniffed() {
    let call = |content_type: &'static str| {
        Request::post("/add")
            .header(header::CONTENT_TYPE, content_type)
            .body(Body::from("1"))
            .expect("request")
    };
    let served = headers(call("application/json")).await;
    assert_eq!(value(&served, "x-content-type-options"), Some("nosniff"));
    let refused = headers(call("text/plain")).await;
    assert_eq!(value(&refused, "x-content-type-options"), Some("nosniff"));
}

#[tokio::test]
async fn a_stream_asks_a_proxy_not_to_buffer_it() {
    let request = Request::get("/ticks?args=1")
        .header(header::ACCEPT, "text/event-stream")
        .header("hypersig-call", "1")
        .body(Body::empty())
        .expect("request");
    let headers = headers(request).await;
    assert_eq!(value(&headers, "x-accel-buffering"), Some("no"));
    assert_eq!(value(&headers, "x-content-type-options"), Some("nosniff"));
}

#[tokio::test]
async fn the_runtime_is_not_sniffed() {
    let headers = headers(get(hypersig::RUNTIME_PATH)).await;
    assert_eq!(value(&headers, "x-content-type-options"), Some("nosniff"));
}
