//! The axum integration, exercised through the router without a socket.

use axum::Router;
use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderName, Request, StatusCode, header};
use futures::{StreamExt as _, stream};
use http_body_util::BodyExt as _;
use hypersig::html::{a, p};
use hypersig::{
    Action, Page, PageResponse, Patches, Rest, Route, RouterExt, Search, Segment, Signals,
    StreamAction,
};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

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
            member_page(
                &format!(
                    "{team} {} {} {:?} {:?}",
                    filter.q, filter.page, filter.tag, filter.tags
                ),
                0,
            )
        };
    const FIND: Route<(), Filter> = "/find" => |(), filter: Filter| async move {
        PageResponse::redirect(TEAM.url(("red team".to_owned(), filter)))
    };
    const TAB: Route<(String, Tab)> = "/tabs/{team}/{tab}" =>
        |(), (team, tab): (String, Tab)| async move { member_page(&format!("{team} {tab:?}"), 0) };
    const FILE: Route<(String, Rest)> = "/repos/{repo}/files/{*path}" =>
        |(), (repo, path): (String, Rest)| async move { member_page(&format!("{repo}:{path}"), 0) };
    const DEEP: Route<(i32, i32, i32, i32, i32, String)> = "/{a}/{b}/{c}/{d}/{e}/{f}" =>
        |(), (a, b, c, d, e, f): (i32, i32, i32, i32, i32, String)| async move {
            member_page(&f, a + b + c + d + e)
        };
    action ADD: Action<Counter, (i32, i32)> = post "/add" =>
        |(), (count, by): (i32, i32)| async move {
            Patches::from(Counter::count().patch(count + by))
        };
    action PEEK: Action<Counter, i32> = get "/peek" => |(), count: i32| async move {
        Patches::from(Counter::count().patch(count))
    };
    action MOVE: Action<Counter, i32> = post "/move" => |(), id: i32| async move {
        Patches::from(Counter::count().patch(id)).navigate(ORG_MEMBER.url(("acme".to_owned(), id)))
    };
    action LENGTH: Action<Counter, String> = get "/length" => |(), text: String| async move {
        Patches::from(Counter::count().patch(i32::try_from(text.chars().count()).unwrap()))
    };
    stream TICKS: StreamAction<Counter, i32> = "/ticks" => |(), from: i32| {
        stream::iter([from, from + 1]).map(|count| Counter::count().patch(count).into())
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

#[derive(Signals, Search, Serialize, Deserialize)]
struct Filter {
    q: String,
    page: i32,
    tag: Option<String>,
    tags: Vec<Tab>,
}

fn member_page(team: &str, id: i32) -> Page<Counter> {
    Page::new("Member", Counter { count: id })
        .child(p().text(format!("{team} #{id}")))
        .child(a().href(HOME.link()).text("Home"))
}

fn app() -> Router {
    pages(Router::new())
        .hypersig_runtime()
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
async fn a_stream_ends_with_hs_end_once_its_handler_finishes() {
    let (status, body) = send(
        Request::get("/ticks?args=4")
            .header("accept", "text/event-stream")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        "event: hs-signals\ndata: {\"count\":4}\n\n\
         event: hs-signals\ndata: {\"count\":5}\n\n\
         event: hs-end\ndata: {}\n\n"
    );
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
            .header("accept", "text/event-stream")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "event: hs-signals\ndata: {\"count\":5}\n\n");
}

#[tokio::test]
async fn the_requests_actions_build_reach_their_handlers_with_the_arguments() {
    let (status, body) = send(ADD.request(&(4, 3))).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "event: hs-signals\ndata: {\"count\":7}\n\n");

    // A text the query has to escape: a space, `&`, `=`, `/`, `?`, `#`, and
    // a character outside ASCII.
    let (status, body) = send(LENGTH.request(&"a b&c=d/e?f#gé".to_owned())).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "event: hs-signals\ndata: {\"count\":14}\n\n");

    let (status, body) = send(TICKS.request(&4)).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.starts_with("event: hs-signals\ndata: {\"count\":4}\n\n"));
}

#[test]
fn the_request_of_a_call_carries_the_headers_the_runtime_sends() {
    let post = ADD.request(&(4, 3));
    assert_eq!(post.method(), "POST");
    assert_eq!(post.headers()[header::ACCEPT], "text/event-stream");
    assert_eq!(post.headers()[header::CONTENT_TYPE], "application/json");
    let get = PEEK.request(&5);
    assert_eq!(get.uri(), "/peek?args=5");
    assert_eq!(get.headers()[header::ACCEPT], "text/event-stream");
    assert!(get.headers().get(header::CONTENT_TYPE).is_none());
}

#[tokio::test]
async fn arguments_of_the_wrong_type_get_400() {
    let (status, body) = send(
        Request::post("/add")
            .header("content-type", "application/json")
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
    let (status, body) = send(
        Request::post("/add")
            .header("content-type", "application/json")
            .body(Body::empty())
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
async fn serves_the_runtime_compressed_with_brotli_to_a_browser_that_accepts_it() {
    let fetch = |accept: &'static str| async move {
        let response = app()
            .oneshot(
                Request::get(hypersig::RUNTIME_PATH)
                    .header(header::ACCEPT_ENCODING, accept)
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("infallible");
        assert_eq!(response.headers()[header::VARY], "accept-encoding");
        let encoding = response
            .headers()
            .get(header::CONTENT_ENCODING)
            .map(|value| value.to_str().expect("ascii").to_owned());
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        (encoding, body)
    };
    let brotli: &[u8] = include_bytes!("../assets/hypersig.js.br");
    for accept in ["gzip, deflate, br, zstd", "BR;q=0.5", "gzip;q=1.0, br;q=1"] {
        let (encoding, body) = fetch(accept).await;
        assert_eq!(encoding.as_deref(), Some("br"), "{accept}");
        assert_eq!(body, brotli, "{accept}");
    }
    for accept in ["gzip, deflate", "br;q=0", "brotli", ""] {
        let (encoding, body) = fetch(accept).await;
        assert_eq!(encoding, None, "{accept}");
        assert_eq!(body, hypersig::RUNTIME_JS.as_bytes(), "{accept}");
    }
}

#[tokio::test]
async fn serves_the_runtime_at_a_path_named_after_its_hash_for_a_year() {
    let hash = hypersig::RUNTIME_PATH
        .strip_prefix("/_hypersig/runtime.")
        .and_then(|rest| rest.strip_suffix(".js"));
    assert!(
        hash.is_some_and(|hash| hash.len() == 16 && hash.bytes().all(|b| b.is_ascii_hexdigit())),
        "{}",
        hypersig::RUNTIME_PATH
    );
    let response = app()
        .oneshot(
            Request::get(hypersig::RUNTIME_PATH)
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("infallible");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::CACHE_CONTROL],
        "public, max-age=31536000, immutable"
    );
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    assert_eq!(body, hypersig::RUNTIME_JS.as_bytes());

    let html = Page::new("Home", Counter { count: 0 }).render();
    assert!(html.contains(&format!(
        r#"<script type="module" src="{}">"#,
        hypersig::RUNTIME_PATH
    )));
    assert!(!html.contains("data-hs-shape"), "{html}");
}

/// The URL of the stream a page served by a debug build reloads from.
#[cfg(debug_assertions)]
fn dev_url(page: &str) -> &str {
    let start = page
        .find(r#" data-hs-dev=""#)
        .expect("a page names the stream")
        + 14;
    let len = page[start..].find('"').expect("the attribute ends");
    &page[start..start + len]
}

#[cfg(debug_assertions)]
#[tokio::test]
async fn a_page_reloads_from_a_server_started_after_the_one_that_rendered_it() {
    let (_, page) = send(Request::get("/").body(Body::empty()).expect("request")).await;
    let url = dev_url(&page);
    assert!(url.starts_with("/_hypersig/dev?since="), "{url}");
    assert!(page.contains(r#" data-hs-shape="count:i32""#), "{page}");

    // The runtime imports the dev script from beside the stream.
    let (status, script) = send(
        Request::get("/_hypersig/dev.js")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(script.starts_with("// hypersig dev script."), "{script}");

    let (status, body) = send(
        Request::get("/_hypersig/dev?since=0")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "event: hs-reload\ndata: {}\n\n");

    // The server that rendered the page holds the stream open, so its body
    // does not end.
    let response = app()
        .oneshot(Request::get(url).body(Body::empty()).expect("request"))
        .await
        .expect("the router is infallible");
    assert_eq!(response.status(), StatusCode::OK);
    let read = tokio::time::timeout(
        std::time::Duration::from_millis(100),
        response.into_body().collect(),
    )
    .await;
    assert!(read.is_err(), "the stream ended");
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
    assert!(body.contains("point to `/members/{id}` is"), "{body}");
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
    assert!(
        body.contains(r#"red team a b&amp;c 3 Some("x") []"#),
        "{body}"
    );
}

#[tokio::test]
async fn a_repeated_key_reads_into_a_list_field_in_order() {
    let (status, body) = send(
        Request::get("/teams/red?tags=Settings&page=1&q=a+b&tags=Overview")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("red a b 1 None [Settings, Overview]"),
        "{body}"
    );
    let (status, _) = send(
        Request::get("/teams/red?page=1&q=a&page=2")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
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

#[tokio::test]
async fn a_rest_segment_reads_the_rest_of_the_path() {
    let (status, body) = send(
        Request::get("/repos/web/files/docs/a%20b/c.md")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("web:docs/a b/c.md"), "{body}");
    assert_eq!(
        FILE.url(("web".to_owned(), Rest::new("docs/a b/c.md")))
            .as_str(),
        "/repos/web/files/docs/a%20b/c.md"
    );
}

#[derive(Segment, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Locale {
    En,
    #[serde(rename = "ja-JP")]
    Ja,
}

#[test]
fn a_renamed_segment_matches_what_serde_writes() {
    for (locale, text) in [(Locale::En, "en"), (Locale::Ja, "ja-JP")] {
        assert_eq!(
            serde_json::to_string(&locale).expect("serializes"),
            format!("\"{text}\"")
        );
        assert_eq!(
            <Locale as hypersig::Params>::from_segments(&[text]),
            Some(locale)
        );
    }
}

/// The name in the `x-visitor` header, refused with a 401 when it is missing.
struct Visitor(String);

impl<St: Sync> FromRequestParts<St> for Visitor {
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(
        parts: &mut Parts,
        _state: &St,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let visitor = parts
            .headers
            .get("x-visitor")
            .and_then(|name| name.to_str().ok())
            .map(|name| Self(name.to_owned()))
            .ok_or((StatusCode::UNAUTHORIZED, "sign in first"));
        std::future::ready(visitor)
    }
}

hypersig::routes! {
    fn guarded_pages(Router<()>);
    const WHO: Route = "/who" => who;
    const BROKEN: Route = "/broken" => |(), ()| async {
        Err::<Page<Counter>, _>((StatusCode::SERVICE_UNAVAILABLE, "the database is down"))
    };
    action GREET: Action<Counter> = post "/greet" => |(), Visitor(name): Visitor, ()| async move {
        Patches::from(Counter::count().patch(i32::try_from(name.len()).unwrap_or(0)))
    };
    action SIGN_IN: Action<Counter> = post "/sign-in" => |(), ()| async {
        (
            [(header::SET_COOKIE, "session=abc; HttpOnly; Path=/")],
            Counter::count().patch(1),
        )
    };
    action FAIL: Action<Counter, i32> = post "/fail" => |(), count: i32| async move {
        let full = (StatusCode::CONFLICT, "the count is full".to_owned());
        let next = (count < 3).then_some(count + 1).ok_or(full)?;
        Ok::<_, (StatusCode, String)>(Counter::count().patch(next))
    };
}

async fn who(
    (): (),
    Visitor(name): Visitor,
    (): (),
) -> ([(HeaderName, &'static str); 1], Page<Counter>) {
    let page = Page::new("Who", Counter { count: 0 }).child(p().text(name));
    ([(header::SET_COOKIE, "seen=1; Path=/")], page)
}

fn guarded() -> Router {
    guarded_pages(Router::new())
}

async fn send_guarded(request: Request<Body>) -> (StatusCode, HeaderMap, String) {
    let response = guarded()
        .oneshot(request)
        .await
        .expect("the router is infallible");
    let status = response.status();
    let headers = response.headers().clone();
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (
        status,
        headers,
        String::from_utf8(body.to_vec()).expect("utf-8 body"),
    )
}

/// A POST of `body` as JSON to `path`, with `headers`.
fn post_json(path: &str, headers: &[(&str, &str)], body: &'static str) -> Request<Body> {
    let mut request = Request::post(path).header("content-type", "application/json");
    for &(name, value) in headers {
        request = request.header(name, value);
    }
    request.body(Body::from(body)).expect("request")
}

#[tokio::test]
async fn a_post_whose_body_is_not_json_gets_415() {
    let (status, body) = send(
        Request::post("/add")
            .header("content-type", "text/plain")
            .body(Body::from("[4,3]"))
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert_eq!(body, "the action's body is not `application/json`");
}

#[tokio::test]
async fn a_json_media_type_with_parameters_is_accepted() {
    let (status, _) = send(
        Request::post("/add")
            .header("content-type", "Application/JSON; charset=utf-8")
            .body(Body::from("[4,3]"))
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn a_call_from_another_site_gets_403() {
    for site in ["cross-site", "same-site"] {
        let (status, body) = send(post_json("/add", &[("sec-fetch-site", site)], "[4,3]")).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{site}");
        assert_eq!(body, "the action was called from another site");
    }
    let (status, body) = send(
        Request::get("/peek?args=5")
            .header("sec-fetch-site", "cross-site")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
}

#[tokio::test]
async fn a_get_the_runtime_did_not_send_gets_403() {
    // What an `<img>` on another site sends from a browser without
    // `Sec-Fetch-Site`.
    let (status, body) = send(
        Request::get("/peek?args=5")
            .header("accept", "image/avif,image/webp,*/*")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(
        body,
        "the call lacks the `Accept` header the hypersig runtime sends"
    );
}

#[tokio::test]
async fn a_call_from_the_same_origin_is_served() {
    let (status, body) = send(post_json(
        "/add",
        &[("sec-fetch-site", "same-origin")],
        "[4,3]",
    ))
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "event: hs-signals\ndata: {\"count\":7}\n\n");
}

#[tokio::test]
async fn an_action_reads_the_request_through_its_extractor() {
    let (status, _, body) =
        send_guarded(post_json("/greet", &[("x-visitor", "Ada")], "null")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "event: hs-signals\ndata: {\"count\":3}\n\n");
}

#[tokio::test]
async fn an_extractor_that_fails_answers_with_its_rejection() {
    let (status, _, body) = send_guarded(post_json("/greet", &[], "null")).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body, "sign in first");
}

#[tokio::test]
async fn an_action_sets_a_cookie_beside_its_patches() {
    let (status, headers, body) = send_guarded(post_json("/sign-in", &[], "null")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers[header::SET_COOKIE], "session=abc; HttpOnly; Path=/");
    assert_eq!(body, "event: hs-signals\ndata: {\"count\":1}\n\n");
}

#[tokio::test]
async fn an_action_that_returns_an_error_answers_with_it() {
    let (status, _, body) = send_guarded(post_json("/fail", &[], "2")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "event: hs-signals\ndata: {\"count\":3}\n\n");
    let (status, _, body) = send_guarded(post_json("/fail", &[], "3")).await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body, "the count is full");
}

#[tokio::test]
async fn a_page_reads_the_request_and_sets_a_cookie() {
    let (status, headers, body) = send_guarded(
        Request::get("/who")
            .header("x-visitor", "Grace")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers[header::SET_COOKIE], "seen=1; Path=/");
    assert!(body.contains("<p>Grace</p>"), "{body}");
    let (status, _, _) =
        send_guarded(Request::get("/who").body(Body::empty()).expect("request")).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn a_page_that_returns_an_error_answers_with_it() {
    let (status, _, body) = send_guarded(
        Request::get("/broken")
            .body(Body::empty())
            .expect("request"),
    )
    .await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body, "the database is down");
}
