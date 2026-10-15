//! The axum integration, exercised through the router without a socket.

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use hypersig::{Action, Patches, RouterExt, Signals};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

const ADD: Action<Counter, (i32, i32)> = Action::post("/add");
const PEEK: Action<Counter, i32> = Action::get("/peek");

fn app() -> Router {
    Router::new()
        .hypersig_runtime()
        .action(ADD, |(), (count, by): (i32, i32)| async move {
            Patches::from(Counter::count().patch(count + by))
        })
        .action(PEEK, |(), count: i32| async move {
            Patches::from(Counter::count().patch(count))
        })
}

async fn send(request: Request<Body>) -> (StatusCode, String) {
    let response = app()
        .oneshot(request)
        .await
        .expect("the router is infallible");
    let status = response.status();
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (
        status,
        String::from_utf8(body.to_vec()).expect("utf-8 body"),
    )
}

#[tokio::test]
async fn post_reads_the_arguments_from_the_body() {
    let (status, body) = send(
        Request::post("/add")
            .header("content-type", "application/json")
            .body(Body::from("[4,3]"))
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "event: hs-signals\ndata: {\"count\":7}\n\n");
}

#[tokio::test]
async fn get_reads_the_arguments_from_the_query() {
    let (status, body) = send(
        Request::get("/peek?args=5")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "event: hs-signals\ndata: {\"count\":5}\n\n");
}

#[tokio::test]
async fn arguments_of_the_wrong_type_get_400() {
    let (status, body) = send(
        Request::post("/add")
            .body(Body::from(r#"["four",3]"#))
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(
        body.starts_with("the action's arguments did not deserialize"),
        "{body}"
    );
}

#[tokio::test]
async fn missing_arguments_get_400() {
    let (status, body) = send(Request::post("/add").body(Body::empty()).expect("request")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(
        body.starts_with("the action's arguments did not deserialize"),
        "{body}"
    );
}

#[tokio::test]
async fn serves_the_runtime() {
    let (status, body) = send(
        Request::get(hypersig::RUNTIME_PATH)
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, hypersig::RUNTIME_JS);
}
