//! The answer to a call whose handler panics.

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use futures::stream;
use hypersig::{Action, Patches, Signals, StreamAction, Upload, Uploaded};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

hypersig::routes! {
    fn calls(Router<()>);
    action FAIL: Action<Counter, i32> = post "/fail" => |(), count: i32| async move {
        assert!(count < 0, "the handler expects a negative count");
        Patches::from(Counter::count().patch(count))
    };
    action SURVIVE: Action<Counter, i32> = post "/survive" => |(), count: i32| async move {
        Patches::from(Counter::count().patch(count))
    };
    stream FAIL_STREAM: StreamAction<Counter, i32> = "/fail-stream" => |(), count: i32| {
        assert!(count < 0, "the handler expects a negative count");
        stream::empty::<Patches<Counter>>()
    };
    upload FAIL_UPLOAD: Upload<Counter> = "/fail-upload" => |(), _: Uploaded<()>| async move {
        panic!("the handler cannot store the file");
        #[expect(unreachable_code, reason = "the handler's return type is inferred from here")]
        Patches::<Counter>::new()
    };
}

async fn status(request: Request<Body>) -> StatusCode {
    calls(Router::new())
        .oneshot(request)
        .await
        .expect("the router is infallible")
        .status()
}

fn post(path: &str) -> Request<Body> {
    Request::post(path)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from("3"))
        .expect("request")
}

#[tokio::test]
async fn an_action_whose_handler_panics_answers_500() {
    assert_eq!(
        status(post("/fail")).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(status(post("/survive")).await, StatusCode::OK);
}

#[tokio::test]
async fn a_stream_whose_handler_panics_answers_500() {
    let request = Request::get("/fail-stream?args=3")
        .header(header::ACCEPT, "text/event-stream")
        .header("hypersig-call", "1")
        .body(Body::empty())
        .expect("request");
    assert_eq!(status(request).await, StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn an_upload_whose_handler_panics_answers_500() {
    let body = "--b\r\nContent-Disposition: form-data; name=\"args\"\r\n\r\nnull\r\n--b--\r\n";
    let request = Request::post("/fail-upload")
        .header(header::CONTENT_TYPE, "multipart/form-data; boundary=b")
        .header("hypersig-upload", "1")
        .body(Body::from(body))
        .expect("request");
    assert_eq!(status(request).await, StatusCode::INTERNAL_SERVER_ERROR);
}
