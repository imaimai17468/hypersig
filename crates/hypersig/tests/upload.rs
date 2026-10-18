//! Uploads served by the axum integration, driven with hand-built requests.

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt as _;
use hypersig::{Patches, RouterExt, Signals, Upload, Uploaded};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Gallery {
    summary: String,
}

const SEND: Upload<Gallery, String> = Upload::post("/send").max_bytes(1024);

fn app() -> Router {
    Router::new().upload(SEND, |(), uploaded: Uploaded<String>| async move {
        let files: Vec<String> = uploaded
            .files()
            .iter()
            .map(|file| {
                format!(
                    "{}:{}:{}",
                    file.file_name().unwrap_or("?"),
                    file.content_type().unwrap_or("?"),
                    String::from_utf8_lossy(file.bytes())
                )
            })
            .collect();
        Patches::from(Gallery::summary().patch(format!("{} {}", uploaded.args(), files.join(","))))
    })
}

const BOUNDARY: &str = "XyZ";

/// A multipart body with an `args` part holding `args` and one `files` part per file.
fn multipart(args: &str, files: &[(&str, &str, &str)]) -> String {
    let parts = files.iter().map(|(name, media, contents)| {
        format!(
            "--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"files\"; filename=\"{name}\"\r\nContent-Type: {media}\r\n\r\n{contents}\r\n"
        )
    });
    format!(
        "--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"args\"; filename=\"blob\"\r\nContent-Type: application/json\r\n\r\n{args}\r\n{}--{BOUNDARY}--\r\n",
        parts.collect::<String>()
    )
}

fn upload(headers: &[(&str, &str)], body: String) -> Request<Body> {
    let mut request = Request::post("/send")
        .header(
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={BOUNDARY}"),
        )
        .header(header::CONTENT_LENGTH, body.len());
    for (name, value) in headers {
        request = request.header(*name, *value);
    }
    request.body(Body::from(body)).expect("request")
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

const FROM_PAGE: (&str, &str) = ("hypersig-upload", "1");

#[tokio::test]
async fn the_handler_receives_the_arguments_and_the_files() {
    let body = multipart(
        "\"beach\"",
        &[
            ("a.png", "image/png", "one"),
            ("b.txt", "text/plain", "two"),
        ],
    );
    let (status, text) = send(upload(&[FROM_PAGE], body)).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        text.contains(r#""summary":"beach a.png:image/png:one,b.txt:text/plain:two""#),
        "{text}"
    );
}

#[tokio::test]
async fn an_upload_without_files_still_reaches_the_handler() {
    let (status, text) = send(upload(&[FROM_PAGE], multipart("\"none\"", &[]))).await;
    assert_eq!(status, StatusCode::OK);
    assert!(text.contains(r#""summary":"none ""#), "{text}");
}

#[tokio::test]
async fn an_upload_without_the_runtime_header_gets_403() {
    let body = multipart("\"x\"", &[("a.txt", "text/plain", "one")]);
    let (status, text) = send(upload(&[], body)).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(
        text,
        "the upload lacks the header the hypersig runtime adds"
    );
}

#[tokio::test]
async fn an_upload_from_another_site_gets_403() {
    let body = multipart("\"x\"", &[]);
    let (status, _) = send(upload(&[FROM_PAGE, ("sec-fetch-site", "cross-site")], body)).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn an_upload_whose_body_is_not_multipart_gets_415() {
    let request = Request::post("/send")
        .header(header::CONTENT_TYPE, "application/json")
        .header(FROM_PAGE.0, FROM_PAGE.1)
        .body(Body::from("\"x\""))
        .expect("request");
    let (status, _) = send(request).await;
    assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

#[tokio::test]
async fn an_upload_longer_than_its_limit_gets_413() {
    let large = "x".repeat(2048);
    let body = multipart("\"x\"", &[("big.txt", "text/plain", &large)]);
    let (status, text) = send(upload(&[FROM_PAGE], body)).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(text, "the upload is longer than its limit");
}

#[tokio::test]
async fn an_upload_longer_than_its_limit_without_a_length_gets_413() {
    let large = "x".repeat(2048);
    let body = multipart("\"x\"", &[("big.txt", "text/plain", &large)]);
    // A body of unknown length, so the request carries no `Content-Length`
    // and the limit is found while reading.
    let chunks: Vec<Result<Vec<u8>, std::io::Error>> = body
        .as_bytes()
        .chunks(256)
        .map(|chunk| Ok(chunk.to_vec()))
        .collect();
    let request = Request::post("/send")
        .header(
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={BOUNDARY}"),
        )
        .header(FROM_PAGE.0, FROM_PAGE.1)
        .body(Body::from_stream(futures::stream::iter(chunks)))
        .expect("request");
    let (status, _) = send(request).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn arguments_that_do_not_deserialize_get_400() {
    let (status, _) = send(upload(&[FROM_PAGE], multipart("7", &[]))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn an_ordinary_action_still_refuses_a_multipart_body() {
    use hypersig::Action;

    const SAVE: Action<Gallery> = Action::post("/save");
    let app: Router =
        Router::new().action(SAVE, |(), (): ()| async move { Patches::<Gallery>::new() });
    let request = Request::post("/save")
        .header(
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={BOUNDARY}"),
        )
        .body(Body::from(multipart("null", &[])))
        .expect("request");
    let response = app.oneshot(request).await.expect("infallible");
    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
}
