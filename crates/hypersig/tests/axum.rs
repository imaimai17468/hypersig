//! The axum integration, exercised through the router without a socket.

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt as _;
use hypersig::html::{a, p};
use hypersig::{Action, Page, PageResponse, Patches, Route, RouterExt, Segment, Signals};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

const ADD: Action<Counter, (i32, i32)> = Action::post("/add");
const PEEK: Action<Counter, i32> = Action::get("/peek");
const MOVE: Action<Counter, i32> = Action::post("/move");

hypersig::routes! {
    fn pages(Router<()>);
    const HOME: Route = "/" => |(), ()| async {
        Page::new("Home", Counter { count: 0 })
            .child(a().href(MEMBER.link_with(("red team", 2))).text("Member"))
            .child(a().href(ORG_MEMBER.link_with(("acme", 9))).text("Org member"))
    };
    const MEMBER: Route<(String, i32)> = "/teams/{team}/members/{id}" =>
        |(), (team, id): (String, i32)| async move { (id >= 0).then(|| member_page(&team, id)) };
    const ORG: Route<String> = "/orgs/{org}" => |(), org: String| async move { member_page(&org, 0) };
    const ORG_MEMBER: Route<(String, i32)> = "/orgs/{org}/members/{id}" in ORG =>
        |(), (org, id): (String, i32)| async move { member_page(&org, id) };
    const OLD_MEMBER: Route<i32> = "/people/{id}" => |(), id: i32| async move {
        PageResponse::redirect(MEMBER.url(("old team".to_owned(), id)))
    };
    const TEAM: Route<String, Filter> = "/teams/{team}" =>
        |(), (team, filter): (String, Filter)| async move {
            member_page(&format!("{team} {} {} {:?}", filter.q, filter.page, filter.tag), 0)
        };
    const FIND: Route<(), Filter> = "/find" => |(), filter: Filter| async move {
        PageResponse::redirect(TEAM.url(("red team".to_owned(), filter)))
    };
    const TAB: Route<(String, Tab)> = "/tabs/{team}/{tab}" =>
        |(), (team, tab): (String, Tab)| async move { member_page(&format!("{team} {tab:?}"), 0) };
    const DEEP: Route<(i32, i32, i32, i32, i32, String)> = "/{a}/{b}/{c}/{d}/{e}/{f}" =>
        |(), (a, b, c, d, e, f): (i32, i32, i32, i32, i32, String)| async move {
            member_page(&f, a + b + c + d + e)
        };
}

hypersig::routes! {
    fn legacy_pages(Router<()>);
    const NESTED: Route<i32> = "/members/{id}" => |(), id: i32| async move { member_page("nested", id) };
}

#[derive(Segment, Serialize, Deserialize, Debug)]
enum Tab {
    Overview,
    Settings,
}

#[derive(Signals, Serialize, Deserialize)]
struct Filter {
    q: String,
    page: i32,
    tag: Option<String>,
}

fn member_page(team: &str, id: i32) -> Page<Counter> {
    Page::new("Member", Counter { count: id })
        .child(p().text(format!("{team} #{id}")))
        .child(a().href(HOME.link()).text("Home"))
}

fn app() -> Router {
    pages(Router::new())
        .hypersig_runtime()
        .action(ADD, |(), (count, by): (i32, i32)| async move {
            Patches::from(Counter::count().patch(count + by))
        })
        .action(PEEK, |(), count: i32| async move {
            Patches::from(Counter::count().patch(count))
        })
        .action(MOVE, |(), id: i32| async move {
            Patches::from(Counter::count().patch(id))
                .navigate(ORG_MEMBER.url(("acme".to_owned(), id)))
        })
        .nest("/legacy", legacy_pages(Router::new()))
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

#[tokio::test]
async fn a_page_handler_redirects_to_another_route() {
    let response = app()
        .oneshot(
            Request::get("/people/4")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("the router is infallible");
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers()["location"],
        "/teams/old%20team/members/4"
    );
}

#[tokio::test]
async fn an_action_sends_the_browser_to_another_page_after_its_patches() {
    let (status, body) = send(
        Request::post("/move")
            .header("content-type", "application/json")
            .body(Body::from("5"))
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        "event: hs-signals\ndata: {\"count\":5}\n\nevent: hs-navigate\ndata: {\"url\":\"/orgs/acme/members/5\"}\n\n"
    );
}

#[tokio::test]
async fn a_page_receives_its_search_parameters() {
    let (status, body) = send(
        Request::get("/teams/red%20team?page=3&q=a%20b%26c&tag=x")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains(r#"red team a b&amp;c 3 Some("x")"#), "{body}");
}

#[tokio::test]
async fn a_query_string_that_does_not_deserialize_gets_400() {
    let (status, _) = send(
        Request::get("/teams/red?page=three&q=a")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn a_redirect_writes_the_search_parameters_and_leaves_out_none() {
    let response = app()
        .oneshot(
            Request::get("/find?q=%C3%A9&page=1")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("the router is infallible");
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers()["location"],
        "/teams/red%20team?page=1&q=%C3%A9"
    );
}

#[tokio::test]
async fn an_enum_segment_parses_by_variant_name() {
    let (status, body) = send(
        Request::get("/tabs/red/Settings")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("red Settings"), "{body}");
    let (status, _) = send(
        Request::get("/tabs/red/settings")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(
        TAB.url(("red".to_owned(), Tab::Overview)).as_str(),
        "/tabs/red/Overview"
    );
}

#[tokio::test]
async fn a_route_takes_six_parameters() {
    let (status, body) = send(
        Request::get("/1/2/3/4/5/six")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("six #15"), "{body}");
}
