//! The axum integration, exercised through the router without a socket.

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use hypersig::html::{a, p};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

const ADD: Action<Counter, (i32, i32)> = Action::post("/add");
const PEEK: Action<Counter, i32> = Action::get("/peek");
const HOME: Route = Route::new("/");
const MEMBER: Route<(String, i32)> = Route::new("/teams/{team}/members/{id}");
const ORG: Route<String> = Route::new("/orgs/{org}");
const ORG_MEMBER: Route<(String, i32)> = ORG.child("/orgs/{org}/members/{id}");
const NESTED: Route<i32> = Route::new("/members/{id}");

fn member_page(team: &str, id: i32) -> Page<Counter> {
    Page::new("Member", Counter { count: id })
        .child(p().text(format!("{team} #{id}")))
        .child(a().href(HOME.link()).text("Home"))
}

fn app() -> Router {
    Router::new()
        .hypersig_runtime()
        .action(ADD, |(), (count, by): (i32, i32)| async move {
            Patches::from(Counter::count().patch(count + by))
        })
        .action(PEEK, |(), count: i32| async move {
            Patches::from(Counter::count().patch(count))
        })
        .page(HOME, |(), ()| async {
            Page::new("Home", Counter { count: 0 })
                .child(a().href(MEMBER.link_with(("red team", 2))).text("Member"))
                .child(
                    a().href(ORG_MEMBER.link_with(("acme", 9)))
                        .text("Org member"),
                )
        })
        .page(MEMBER, |(), (team, id): (String, i32)| async move {
            (id >= 0).then(|| member_page(&team, id))
        })
        .page(ORG_MEMBER, |(), (org, id): (String, i32)| async move {
            member_page(&org, id)
        })
        .nest(
            "/legacy",
            Router::new().page(
                NESTED,
                |(), id: i32| async move { member_page("nested", id) },
            ),
        )
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

#[tokio::test]
async fn serves_a_page_without_parameters() {
    let (status, body) = send(Request::get("/").body(Body::empty()).expect("request")).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains(r#"href="/teams/red%20team/members/2""#),
        "{body}"
    );
}

#[tokio::test]
async fn a_page_receives_its_decoded_parameters() {
    let (status, body) = send(
        Request::get("/teams/red%20team/members/2")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("red team #2"), "{body}");
}

#[tokio::test]
async fn a_segment_that_does_not_parse_gets_404() {
    let (status, _) = send(
        Request::get("/teams/red/members/two")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn a_handler_that_answers_none_gets_404() {
    let (status, _) = send(
        Request::get("/teams/red/members/-1")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn a_child_page_is_served_where_its_link_points() {
    let (_, home) = send(Request::get("/").body(Body::empty()).expect("request")).await;
    assert!(home.contains(r#"href="/orgs/acme/members/9""#), "{home}");
    let (status, body) = send(
        Request::get("/orgs/acme/members/9")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("acme #9"), "{body}");
}

#[tokio::test]
async fn a_page_on_a_nested_router_answers_with_a_500() {
    let (status, body) = send(
        Request::get("/legacy/members/9")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert!(body.contains("`/legacy/members/{id}`"), "{body}");
}
