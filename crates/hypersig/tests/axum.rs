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

#[derive(Serialize, Deserialize)]
struct By(i32);

const ADD: Action<Counter, By> = Action::post("/add");

fn app() -> Router {
    Router::new()
        .hypersig_runtime()
        .action(ADD, |(), signals: Counter, By(by): By| async move {
            Patches::from(Counter::count().patch(signals.count + by))
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
async fn action_answers_with_patches() {
    let (status, body) = send(
        Request::post("/add?args=3")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"count":4}"#))
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "event: hs-signals\ndata: {\"count\":7}\n\n");
}

#[tokio::test]
async fn signals_that_do_not_deserialize_get_400() {
    let (status, body) = send(
        Request::post("/add?args=1")
            .body(Body::from(r#"{"count":"four"}"#))
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(
        body.starts_with("the request's signals did not deserialize"),
        "{body}"
    );
}

#[tokio::test]
async fn missing_arguments_get_400() {
    let (status, body) = send(
        Request::post("/add")
            .body(Body::from(r#"{"count":1}"#))
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body.starts_with("the request's action arguments"), "{body}");
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
