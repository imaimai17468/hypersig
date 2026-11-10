//! Event streams sent without chunked encoding, for the Linux web view.

use axum::Router;
use axum::extract::Request;
use axum::http::{HeaderValue, Version, header};
use axum::middleware::{self, Next};
use axum::response::Response;

/// `router` with each event stream it answers sent as HTTP/1.0.
pub(crate) fn wrap(router: Router) -> Router {
    router.layer(middleware::from_fn(unchunk))
}

// WebKitGTK 2.52 with libsoup 3.4 passes a page one read of a chunked body
// each time bytes arrive, 128 bytes with the chunk's header, and can keep
// the rest of an event until the next bytes arrive. hyper sends an HTTP/1.0
// answer without a length as the bytes up to the connection's close, which
// the web view passes on as they arrive.
async fn unchunk(request: Request, next: Next) -> Response {
    let mut answer = next.run(request).await;
    if answer
        .headers()
        .get(header::CONTENT_TYPE)
        .is_some_and(is_event_stream)
    {
        *answer.version_mut() = Version::HTTP_10;
    }
    answer
}

fn is_event_stream(value: &HeaderValue) -> bool {
    value.as_bytes().starts_with(b"text/event-stream")
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::io::{Read as _, Write as _};
    use std::net::TcpStream;
    use std::time::Duration;

    use axum::Router;
    use axum::body::Body;
    use axum::http::header;
    use axum::routing::get;
    use tokio::net::TcpListener;

    use super::wrap;

    /// The raw answer the server writes to an HTTP/1.1 request for `path`
    /// with `connection` as its `Connection` header, read until the server
    /// closes the connection.
    async fn answer(path: &'static str, connection: &'static str) -> String {
        let router = Router::new()
            .route(
                "/events",
                get(|| async {
                    let events = tokio_stream::once(Ok::<_, Infallible>("data: 1\n\n"));
                    (
                        [(header::CONTENT_TYPE, "text/event-stream")],
                        Body::from_stream(events),
                    )
                }),
            )
            .route(
                "/page",
                get(|| async {
                    Body::from_stream(tokio_stream::once(Ok::<_, Infallible>("page")))
                }),
            );
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, wrap(router)).await });
        tokio::task::spawn_blocking(move || {
            let mut stream = TcpStream::connect(address).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            write!(
                stream,
                "GET {path} HTTP/1.1\r\nHost: {address}\r\nConnection: {connection}\r\n\r\n"
            )
            .unwrap();
            let mut answer = String::new();
            stream.read_to_string(&mut answer).unwrap();
            answer
        })
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn an_event_stream_is_sent_until_the_connection_closes() {
        // The web view keeps its connections alive, so the server is the
        // one that closes this one.
        let answer = answer("/events", "keep-alive").await;
        assert!(answer.starts_with("HTTP/1.0 200"), "{answer}");
        assert!(
            !answer.to_ascii_lowercase().contains("transfer-encoding"),
            "{answer}"
        );
        assert!(answer.ends_with("\r\n\r\ndata: 1\n\n"), "{answer}");
    }

    #[tokio::test]
    async fn another_answer_keeps_http_1_1() {
        let answer = answer("/page", "close").await;
        assert!(answer.starts_with("HTTP/1.1 200"), "{answer}");
        assert!(answer.contains("transfer-encoding: chunked"), "{answer}");
    }
}
