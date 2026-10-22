//! An application served under a path prefix, inside `Router::nest` or behind a proxy that strips it.

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt as _;
use hypersig::html::{a, button, p};
use hypersig::{
    Action, Base, Page, PageResponse, Patches, Route, RouterExt, Search, Signals, StreamAction,
    Unserved,
};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Shop {
    count: i32,
}

const APP: Base = Base::new("/app");

#[derive(Signals, Search, Serialize, Deserialize)]
struct Filter {
    color: Vec<String>,
    q: Option<String>,
}

hypersig::routes! {
    fn pages(Router<()>) under APP else missing;
    const HOME: Route = "/" => |(), ()| async {
        shop("Home")
            .child(a().href(ITEM.link_with(3)).text("Tea"))
            .child(a().href(ITEM.link_with(Shop::count())).text("Counted"))
            .child(button().on_click(ADD.call_with(Shop::count())).text("Add"))
            .child(button().on_load(LIVE.call()).text("Live"))
    };
    const ITEM: Route<i32> = "/items/{id}" => |(), id: i32| async move {
        (id < 10).then(|| shop("Item").child(a().href(HOME.link()).text("Home")))
    };
    const REVIEW: Route<(i32, i32)> = "/items/{id}/reviews/{review}" in ITEM =>
        |(), (id, review): (i32, i32)| async move { shop(&format!("Review {review} of {id}")) };
    const FIND: Route<(), Filter> = "/find" => |(), filter: Filter| async move {
        let fixed = Filter { color: vec!["a b".to_owned(), "c".to_owned()], q: None };
        shop(&format!("{:?} {:?}", filter.color, filter.q))
            .child(a().href(FIND.link_with(&fixed)).text("Fixed"))
    };
    const OLD: Route<i32> = "/old/{id}" => |(), id: i32| async move {
        PageResponse::redirect(ITEM.url(id))
    };
}

const ADD: Action<Shop, i32> = Action::post("/add").under(APP);
const LIVE: StreamAction<Shop> = StreamAction::new("/live").under(APP);

fn shop(title: &str) -> Page<Shop> {
    Page::new(title, Shop { count: 0 }).child(p().text(title))
}

async fn missing((): (), unserved: Unserved) -> Page<Shop> {
    shop(&unserved.to_string())
}

/// The application as it is written, with each path registered without the base.
fn app() -> Router {
    pages(Router::new())
        .hypersig_runtime()
        .action(ADD, |(), count: i32| async move {
            Patches::from(Shop::count().patch(count + 1)).navigate(REVIEW.url((count, 1)))
        })
}

async fn get(app: Router, path: &str) -> (StatusCode, String, Option<String>) {
    let response = app
        .oneshot(Request::get(path).body(Body::empty()).expect("request"))
        .await
        .expect("the router is infallible");
    let status = response.status();
    let location = response
        .headers()
        .get(header::LOCATION)
        .map(|value| value.to_str().expect("ASCII").to_owned());
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (
        status,
        String::from_utf8(body.to_vec()).expect("utf-8"),
        location,
    )
}

#[test]
fn links_and_urls_start_with_the_base_and_the_router_paths_do_not() {
    assert_eq!(HOME.url(()).as_str(), "/app");
    assert_eq!(ITEM.url(3).as_str(), "/app/items/3");
    assert_eq!(REVIEW.url((3, 4)).as_str(), "/app/items/3/reviews/4");
    assert_eq!(ITEM.path(), "/items/{id}");
    assert_eq!(REVIEW.path(), "/items/{id}/reviews/{review}");
    assert_eq!(ADD.path(), "/add");
    assert_eq!(APP.as_str(), "/app");
    assert_eq!(Base::ROOT.as_str(), "");
}

async fn check_home(app: Router, home: &str) {
    let (status, body, _) = get(app, home).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(body.contains(r#"href="/app/items/3""#), "{body}");
    // A debug build's page reads the stream that reloads it under the base,
    // as it reads the runtime.
    #[cfg(debug_assertions)]
    assert!(
        body.contains(r#" data-hs-dev="/app/_hypersig/dev?since="#),
        "{body}"
    );
    assert!(body.contains("&quot;/app/items/&quot;"), "{body}");
    assert!(
        body.contains("&quot;POST&quot;,&quot;/app/add&quot;"),
        "{body}"
    );
    assert!(
        body.contains("&quot;GET&quot;,&quot;/app/live&quot;"),
        "{body}"
    );
    let runtime = format!(r#"src="/app{}""#, hypersig::RUNTIME_PATH);
    assert!(body.contains(&runtime), "{body}");
}

#[tokio::test]
async fn a_list_search_field_repeats_its_key_after_the_base_and_reads_back() {
    let filter = Filter {
        color: vec!["red".to_owned(), "dark blue".to_owned()],
        q: Some("x".to_owned()),
    };
    assert_eq!(
        FIND.url(filter).as_str(),
        "/app/find?color=red&color=dark%20blue&q=x"
    );
    let empty = Filter {
        color: Vec::new(),
        q: None,
    };
    assert_eq!(FIND.url(empty).as_str(), "/app/find");

    let served = || Router::new().nest(APP.as_str(), app());
    let (status, body, _) = get(served(), "/app/find?color=red&q=x&color=dark%20blue").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(
        body.contains(r#"<p>["red", "dark blue"] Some("x")</p>"#),
        "{body}"
    );
    assert!(
        body.contains(r#"href="/app/find?color=a%20b&amp;color=c""#),
        "{body}"
    );

    let (status, body, _) = get(served(), "/app/find?q=x&q=y").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body.contains("duplicate field `q`"), "{body}");
}

#[tokio::test]
async fn a_nested_router_serves_the_pages_where_their_links_point() {
    let served = || Router::new().nest(APP.as_str(), app());
    check_home(served(), "/app").await;
    let (status, body, _) = get(served(), "/app/items/3/reviews/4").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Review 4 of 3"), "{body}");
    let (_, item, _) = get(served(), "/app/items/3").await;
    assert!(item.contains(r#"href="/app""#), "{item}");
    let runtime = format!("/app{}", hypersig::RUNTIME_PATH);
    let (status, body, _) = get(served(), &runtime).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, hypersig::RUNTIME_JS);
}

#[tokio::test]
async fn a_router_behind_a_proxy_that_strips_the_base_serves_the_same_pages() {
    check_home(app(), "/").await;
    let (status, body, _) = get(app(), "/items/3/reviews/4").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Review 4 of 3"), "{body}");
}

#[tokio::test]
async fn a_redirect_and_a_navigation_point_under_the_base() {
    let (status, _, location) = get(Router::new().nest(APP.as_str(), app()), "/app/old/5").await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location.as_deref(), Some("/app/items/5"));

    let response = Router::new()
        .nest(APP.as_str(), app())
        .oneshot(
            Request::post("/app/add")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("2"))
                .expect("request"),
        )
        .await
        .expect("the router is infallible");
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let body = String::from_utf8(body.to_vec()).expect("utf-8");
    assert!(
        body.contains(r#"data: {"url":"/app/items/2/reviews/1"}"#),
        "{body}"
    );
}

#[tokio::test]
async fn the_page_for_a_missing_path_loads_the_runtime_under_the_base() {
    let (status, body, _) = get(Router::new().nest(APP.as_str(), app()), "/app/nowhere").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains("no page is served at `/nowhere`"), "{body}");
    let runtime = format!(r#"src="/app{}""#, hypersig::RUNTIME_PATH);
    assert!(body.contains(&runtime), "{body}");
    let (status, _, _) = get(Router::new().nest(APP.as_str(), app()), "/app/items/12").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn a_router_nested_under_another_path_answers_with_a_500_that_names_both() {
    let (status, body, _) = get(Router::new().nest("/shop", app()), "/shop/items/3").await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert!(body.contains("`/items/{id}`"), "{body}");
    assert!(body.contains("`/app`"), "{body}");
    assert!(body.contains("`/shop/items/{id}`"), "{body}");
}

#[test]
fn a_history_entry_points_under_the_base() {
    let click = button::<Shop>()
        .on_click(ITEM.link_with(4).push_url())
        .render();
    assert!(click.contains("&quot;/app/items/4&quot;"), "{click}");
    let patches: Patches<Shop> = Patches::new()
        .push_url(ITEM.url(4))
        .replace_url(HOME.url(()));
    let stream = patches.to_event_stream();
    assert!(
        stream.contains(r#""/app/items/4""#) && stream.contains(r#""/app""#),
        "{stream}"
    );
}
