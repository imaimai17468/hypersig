//! A part embedded at different fields of different pages, answered by one
//! handler per action, exercised through the router without a socket.

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use futures::{StreamExt as _, stream};
use http_body_util::BodyExt as _;
use hypersig::html::{El, Id, button, div, input, p, part, span};
use hypersig::{Action, Fragment, Page, Part, Patches, RouterExt, Signals, StreamAction};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize, Default)]
struct Newsletter {
    email: String,
    status: String,
}

const JOIN: Action<Newsletter, String> = Action::post("/join");
const READERS: StreamAction<Newsletter> = StreamAction::new("/readers");
const BROADCAST: Action<Newsletter> = Action::post("/broadcast");
const BROADCASTS: StreamAction<Newsletter> = StreamAction::new("/broadcasts");

/// A batch built on another thread, as a task that broadcasts one batch to
/// every page builds it, outside any part.
fn broadcast() -> Patches<Newsletter> {
    std::thread::spawn(|| Patches::new().element(&Joined { readers: 3 }))
        .join()
        .expect("the thread renders")
}

/// The line under the form, which shows the status signal.
struct Joined {
    readers: usize,
}

impl Fragment for Joined {
    type Signals = Newsletter;
    const ID: Id = Id::new("joined");

    fn render(&self) -> El<Newsletter> {
        p().text(format!("{} readers. ", self.readers))
            .child(span().bind_text(Newsletter::status()))
    }
}

impl Part for Newsletter {
    fn render() -> El<Self> {
        div()
            .child(input().bind(Newsletter::email()))
            .child(button().on_click(JOIN.call_with(Newsletter::email())))
            .child(hypersig::fragment(&Joined { readers: 0 }))
    }
}

/// A shell that holds the newsletter, so the newsletter sits two fields deep.
#[derive(Signals, Serialize, Deserialize, Default)]
struct Shell {
    footer: Newsletter,
}

impl Part for Shell {
    fn render() -> El<Self> {
        div().child(part(Shell::footer()))
    }
}

#[derive(Signals, Serialize, Deserialize, Default)]
struct Shop {
    footer: Newsletter,
    count: i32,
}

#[derive(Signals, Serialize, Deserialize, Default)]
struct Blog {
    shell: Shell,
}

async fn join((): (), email: String) -> Patches<Newsletter> {
    Patches::new()
        .signals(Newsletter::status().patch(format!("Joined as {email}")))
        .element(&Joined { readers: 1 })
}

fn app() -> Router {
    Router::new()
        .action(JOIN, join)
        .stream(READERS, |(), (): ()| {
            // The stream builds each batch as it is read, after the handler returned.
            stream::iter([2]).map(|readers| Patches::new().element(&Joined { readers }))
        })
        .action(BROADCAST, |(), (): ()| async { broadcast() })
        .stream(BROADCASTS, |(), (): ()| stream::once(async { broadcast() }))
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

fn join_from(part: Option<&str>) -> Request<Body> {
    let request = Request::post("/join").header("content-type", "application/json");
    let request = match part {
        Some(part) => request.header("hypersig-part", part),
        None => request,
    };
    request.body(Body::from("\"ada\"")).expect("request")
}

#[test]
fn a_part_writes_its_paths_under_the_field_it_is_embedded_at() {
    let shop = Page::new("Shop", Shop::default())
        .child(part(Shop::footer()))
        .render();
    assert!(
        shop.contains("[&quot;sig&quot;,&quot;footer.status&quot;]"),
        "{shop}"
    );
    assert!(
        shop.contains("[&quot;footer.email&quot;,&quot;text&quot;]"),
        "{shop}"
    );
    assert!(
        shop.contains("{&quot;part&quot;:&quot;footer&quot;}"),
        "{shop}"
    );

    let blog = Page::new("Blog", Blog::default())
        .child(part(Blog::shell()))
        .render();
    assert!(
        blog.contains("[&quot;sig&quot;,&quot;shell.footer.status&quot;]"),
        "{blog}"
    );
    assert!(
        blog.contains("{&quot;part&quot;:&quot;shell.footer&quot;}"),
        "{blog}"
    );
}

#[tokio::test]
async fn one_handler_answers_each_page_under_its_field() {
    let (status, body) = send(join_from(Some("footer"))).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains(r#"data: {"footer.status":"Joined as ada"}"#),
        "{body}"
    );
    assert!(
        body.contains("[&quot;sig&quot;,&quot;footer.status&quot;]"),
        "{body}"
    );

    let (_, body) = send(join_from(Some("shell.footer"))).await;
    assert!(
        body.contains(r#"data: {"shell.footer.status":"Joined as ada"}"#),
        "{body}"
    );
    assert!(
        body.contains("[&quot;sig&quot;,&quot;shell.footer.status&quot;]"),
        "{body}"
    );
}

#[tokio::test]
async fn a_call_from_outside_a_part_is_answered_from_the_root() {
    let (status, body) = send(join_from(None)).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains(r#"data: {"status":"Joined as ada"}"#),
        "{body}"
    );
}

#[tokio::test]
async fn a_part_header_that_is_not_a_field_path_gets_400() {
    for part in ["", "footer..status", "footer.st-atus", ".footer"] {
        let (status, body) = send(join_from(Some(part))).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{part:?}");
        assert_eq!(body, "the call's part header is not a dotted field path");
    }
}

#[tokio::test]
async fn a_stream_builds_its_fragments_under_the_part() {
    let (status, body) = send(
        Request::get("/readers")
            .header("accept", "text/event-stream")
            .header("hypersig-part", "footer")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("2 readers."), "{body}");
    assert!(
        body.contains("[&quot;sig&quot;,&quot;footer.status&quot;]"),
        "{body}"
    );
}

fn broadcast_from(path: &str, part: Option<&str>) -> Request<Body> {
    let request = if path == "/broadcast" {
        Request::post(path).header("content-type", "application/json")
    } else {
        Request::get(path).header("accept", "text/event-stream")
    };
    let request = match part {
        Some(part) => request.header("hypersig-part", part),
        None => request,
    };
    let body = if path == "/broadcast" { "null" } else { "" };
    request.body(Body::from(body)).expect("request")
}

#[tokio::test]
async fn a_batch_built_outside_the_called_part_gets_500() {
    let (status, body) = send(broadcast_from("/broadcast", Some("footer"))).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert!(
        body.contains("render a part's fragment while its handler or stream runs"),
        "{body}"
    );

    let (status, body) = send(broadcast_from("/broadcast", None)).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("3 readers."), "{body}");
}

#[tokio::test]
async fn a_stream_ends_at_a_batch_built_outside_the_called_part() {
    let response = app()
        .oneshot(broadcast_from("/broadcasts", Some("footer")))
        .await
        .expect("the router is infallible");
    assert_eq!(response.status(), StatusCode::OK);
    let error = response
        .into_body()
        .collect()
        .await
        .expect_err("the stream ends with an error");
    assert!(
        error
            .to_string()
            .contains("render a part's fragment while its handler or stream runs"),
        "{error}"
    );

    let (status, body) = send(broadcast_from("/broadcasts", None)).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("3 readers."), "{body}");
}
