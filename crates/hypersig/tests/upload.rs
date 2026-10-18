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
async fn an_upload_skips_a_part_other_than_a_file() {
    let body = format!(
        "--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"args\"\r\n\r\n\"mixed\"\r\n\
         --{BOUNDARY}\r\nContent-Disposition: form-data; name=\"note\"\r\n\r\nnot a file\r\n\
         --{BOUNDARY}\r\nContent-Disposition: form-data; name=\"files\"; filename=\"a.txt\"\r\nContent-Type: text/plain\r\n\r\none\r\n\
         --{BOUNDARY}--\r\n"
    );
    let (status, text) = send(upload(&[FROM_PAGE], body)).await;
    assert_eq!(status, StatusCode::OK, "{text}");
    assert!(
        text.contains(r#""summary":"mixed a.txt:text/plain:one""#),
        "{text}"
    );
}

#[tokio::test]
async fn an_upload_whose_first_part_is_not_the_arguments_gets_400() {
    let body = format!(
        "--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"files\"; filename=\"a.txt\"\r\nContent-Type: text/plain\r\n\r\none\r\n\
         --{BOUNDARY}\r\nContent-Disposition: form-data; name=\"args\"\r\n\r\n\"late\"\r\n\
         --{BOUNDARY}--\r\n"
    );
    let (status, text) = send(upload(&[FROM_PAGE], body)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(text, "the upload's first part is not the arguments");
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

mod streaming {
    use std::sync::atomic::{AtomicBool, Ordering};

    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode, header};
    use futures::StreamExt as _;
    use http_body_util::BodyExt as _;
    use hypersig::{Patches, RequestError, RouterExt, Upload, UploadStream};
    use tower::ServiceExt as _;

    use super::{BOUNDARY, FROM_PAGE, Gallery, multipart};

    const STREAM: Upload<Gallery, String> = Upload::post("/stream").max_bytes(64 * 1024);

    /// Set once the handler has read a chunk, so a refusal can be told apart
    /// from one made before the handler ran.
    static READ_A_CHUNK: AtomicBool = AtomicBool::new(false);

    /// Answers with the arguments and, per file, its name, its length, and
    /// how many chunks it arrived in.
    fn app() -> Router {
        Router::new().upload_stream(STREAM, |(), mut upload: UploadStream<String>| async move {
            let mut files = Vec::new();
            while let Some(mut file) = upload.next_file().await? {
                let name = file.file_name().unwrap_or("?").to_owned();
                let (mut length, mut chunks) = (0, 0);
                while let Some(chunk) = file.chunk().await? {
                    READ_A_CHUNK.store(true, Ordering::SeqCst);
                    length += chunk.len();
                    chunks += 1;
                }
                files.push(format!("{name}:{length}:{chunks}"));
            }
            let summary = format!("{} {}", upload.args(), files.join(","));
            Ok::<_, RequestError>(Patches::from(Gallery::summary().patch(summary)))
        })
    }

    /// A request whose body arrives in chunks of `chunk` bytes, with no `Content-Length`.
    fn chunked(headers: &[(&str, &str)], body: &str, chunk: usize) -> Request<Body> {
        let chunks: Vec<Result<Vec<u8>, std::io::Error>> = body
            .as_bytes()
            .chunks(chunk)
            .map(|part| Ok(part.to_vec()))
            .collect();
        let mut request = Request::post("/stream").header(
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={BOUNDARY}"),
        );
        for (name, value) in headers {
            request = request.header(*name, *value);
        }
        // Each chunk waits for the next poll, as one from the network would,
        // so the parser cannot read the whole body at once.
        let arriving = futures::stream::iter(chunks).then(|chunk| async move {
            tokio::task::yield_now().await;
            chunk
        });
        request.body(Body::from_stream(arriving)).expect("request")
    }

    async fn send(request: Request<Body>) -> (StatusCode, String) {
        let response = app().oneshot(request).await.expect("infallible");
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
    async fn the_handler_reads_each_file_in_chunks() {
        let large = "x".repeat(40 * 1024);
        let body = multipart(
            "\"beach\"",
            &[
                ("a.bin", "application/octet-stream", &large),
                ("b.txt", "text/plain", "two"),
            ],
        );
        let (status, text) = send(chunked(&[FROM_PAGE], &body, 1024)).await;
        assert_eq!(status, StatusCode::OK, "{text}");
        let summary = text
            .split(r#""summary":""#)
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .expect("a summary patch");
        let (args, files) = summary.split_once(' ').expect("arguments and files");
        assert_eq!(args, "beach");
        let files: Vec<&str> = files.split(',').collect();
        let first: Vec<&str> = files[0].split(':').collect();
        assert_eq!(first[..2], ["a.bin", "40960"]);
        assert!(
            first[2].parse::<usize>().expect("a chunk count") > 1,
            "a 40 KiB file arrived in one chunk: {summary}"
        );
        assert_eq!(files[1], "b.txt:3:1");
    }

    #[tokio::test]
    async fn a_body_past_the_limit_fails_the_read_with_413() {
        let large = "x".repeat(100 * 1024);
        let body = multipart(
            "\"big\"",
            &[("big.bin", "application/octet-stream", &large)],
        );
        let (status, text) = send(chunked(&[FROM_PAGE], &body, 4096)).await;
        assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
        assert_eq!(text, "the upload is longer than its limit");
        assert!(
            READ_A_CHUNK.load(Ordering::SeqCst),
            "the limit was met before the handler read the file"
        );
    }

    #[tokio::test]
    async fn a_body_whose_first_part_is_a_file_gets_400() {
        let body = format!(
            "--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"files\"; filename=\"a.txt\"\r\nContent-Type: text/plain\r\n\r\none\r\n--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"args\"\r\n\r\n\"late\"\r\n--{BOUNDARY}--\r\n"
        );
        let (status, text) = send(chunked(&[FROM_PAGE], &body, 64)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(text, "the upload's first part is not the arguments");
    }

    #[tokio::test]
    async fn a_part_other_than_a_file_is_skipped() {
        let body = format!(
            "--{BOUNDARY}\r\nContent-Disposition: form-data; name=\"args\"\r\n\r\n\"mixed\"\r\n\
             --{BOUNDARY}\r\nContent-Disposition: form-data; name=\"files\"; filename=\"a.txt\"\r\nContent-Type: text/plain\r\n\r\none\r\n\
             --{BOUNDARY}\r\nContent-Disposition: form-data; name=\"note\"\r\n\r\nnot a file\r\n\
             --{BOUNDARY}\r\nContent-Disposition: form-data; name=\"files\"; filename=\"b.txt\"\r\nContent-Type: text/plain\r\n\r\ntwo!\r\n\
             --{BOUNDARY}--\r\n"
        );
        let (status, text) = send(chunked(&[FROM_PAGE], &body, 16)).await;
        assert_eq!(status, StatusCode::OK, "{text}");
        assert!(
            text.contains(r#""summary":"mixed a.txt:3:1,b.txt:4:1""#),
            "{text}"
        );
    }

    #[tokio::test]
    async fn a_streaming_upload_whose_body_is_not_multipart_gets_415() {
        let request = Request::post("/stream")
            .header(header::CONTENT_TYPE, "text/plain")
            .header(FROM_PAGE.0, FROM_PAGE.1)
            .body(Body::from("hello"))
            .expect("request");
        let (status, text) = send(request).await;
        assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
        assert_eq!(text, "the upload's body is not `multipart/form-data`");
    }

    #[tokio::test]
    async fn a_streaming_upload_that_says_it_is_too_long_gets_413() {
        let body = multipart("\"x\"", &[]);
        let request = Request::post("/stream")
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={BOUNDARY}"),
            )
            .header(header::CONTENT_LENGTH, 1024 * 1024)
            .header(FROM_PAGE.0, FROM_PAGE.1)
            .body(Body::from(body))
            .expect("request");
        let (status, _) = send(request).await;
        assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn a_streaming_upload_without_the_runtime_header_gets_403() {
        let body = multipart("\"x\"", &[("a.txt", "text/plain", "one")]);
        let (status, _) = send(chunked(&[], &body, 64)).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn a_streaming_upload_from_another_site_gets_403() {
        let body = multipart("\"x\"", &[]);
        let headers = [FROM_PAGE, ("sec-fetch-site", "cross-site")];
        let (status, _) = send(chunked(&headers, &body, 64)).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
    }
}
