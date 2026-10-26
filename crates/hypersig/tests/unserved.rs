//! The page a router shows where it serves no page, and the status a page can carry.

use axum::Router;
use axum::body::Body;
use axum::http::{HeaderMap, Request, StatusCode, header};
use http_body_util::BodyExt as _;
use hypersig::html::p;
use hypersig::{Page, PageResponse, RequestKind, Route, Search, Segment, Signals, Unserved};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Shop {
    text: String,
}

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    Ja,
}

#[derive(Signals, Search, Serialize, Deserialize)]
struct Sort {
    by: Order,
}

#[derive(Segment, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Order {
    Name,
    Price,
}

hypersig::routes! {
    fn pages(Router<()>) else missing;
    const HOME: Route<Lang> = "/{lang}" => |(), lang: Lang| async move { shop(&format!("{lang:?} home")) };
    const ORDER: Route<(Lang, u32)> = "/{lang}/orders/{id}" in HOME =>
        |(), (_lang, id): (Lang, u32)| async move { (id < 10).then(|| shop(&format!("order {id}"))) };
    const PRODUCTS: Route<Lang, Sort> = "/{lang}/products" in HOME =>
        |(), (_lang, sort): (Lang, Sort)| async move { shop(&format!("by {:?}", sort.by)) };
    const GONE: Route<Lang> = "/{lang}/gone" in HOME => |(), _lang: Lang| async move {
        PageResponse::with_status(StatusCode::GONE, shop("This shop has closed."))
    };
}

fn shop(text: &str) -> Page<Shop> {
    Page::new(
        text,
        Shop {
            text: text.to_owned(),
        },
    )
    .child(p().bind_text(Shop::text()))
}

/// Shows the page in the language of the path, or of the browser where the path names none.
async fn missing((): (), headers: HeaderMap, unserved: Unserved) -> Page<Shop> {
    let lang = unserved.within(HOME).unwrap_or_else(|| {
        let accept = headers.get(header::ACCEPT_LANGUAGE);
        if accept.is_some_and(|value| value.as_bytes().starts_with(b"ja")) {
            Lang::Ja
        } else {
            Lang::En
        }
    });
    let text = match (lang, unserved.status()) {
        (Lang::Ja, StatusCode::BAD_REQUEST) => format!("リクエストが正しくありません: {unserved}"),
        (Lang::Ja, _) => "ページが見つかりません。".to_owned(),
        (Lang::En, StatusCode::BAD_REQUEST) => format!("Bad request: {unserved}"),
        (Lang::En, _) => "No such page.".to_owned(),
    };
    shop(&text)
}

async fn get(app: Router, request: Request<Body>) -> (StatusCode, String) {
    let response = app
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

fn page(path: &str) -> Request<Body> {
    Request::get(path).body(Body::empty()).expect("request")
}

#[tokio::test]
async fn a_path_no_route_matches_gets_the_page_in_the_language_of_its_first_segment() {
    let (status, body) = get(pages(Router::new()), page("/ja/nowhere")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains(">ページが見つかりません。</p>"), "{body}");
}

#[tokio::test]
async fn a_path_without_a_language_gets_the_page_in_the_browsers_language() {
    let request = Request::get("/nowhere")
        .header(header::ACCEPT_LANGUAGE, "ja-JP,ja;q=0.9")
        .body(Body::empty())
        .expect("request");
    let (status, body) = get(pages(Router::new()), request).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains("ページが見つかりません。"), "{body}");
}

#[tokio::test]
async fn a_segment_that_does_not_parse_gets_the_page() {
    let (status, body) = get(pages(Router::new()), page("/en/orders/abc")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains(">No such page.</p>"), "{body}");
    let (status, body) = get(pages(Router::new()), page("/fr")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains(">No such page.</p>"), "{body}");
}

#[tokio::test]
async fn a_segment_or_a_query_value_that_is_not_utf_8_gets_the_page() {
    let (status, body) = get(pages(Router::new()), page("/en/orders/%FF")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains(">No such page.</p>"), "{body}");
    let (status, body) = get(pages(Router::new()), page("/en/products?by=%FF")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body.contains("does not decode to UTF-8"), "{body}");
}

#[tokio::test]
async fn a_handler_that_answers_none_gets_the_page() {
    let (status, body) = get(pages(Router::new()), page("/ja/orders/12")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains("ページが見つかりません。"), "{body}");
    let (status, body) = get(pages(Router::new()), page("/ja/orders/3")).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains(">order 3</p>"), "{body}");
}

#[tokio::test]
async fn a_query_that_does_not_deserialize_gets_the_page_with_400() {
    let (status, body) = get(pages(Router::new()), page("/en/products?by=bogus")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(
        body.contains(">Bad request: Failed to deserialize query string"),
        "{body}"
    );
    let (status, _) = get(pages(Router::new()), page("/en/products?by=price")).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn a_call_from_the_runtime_to_a_missing_path_gets_a_bare_404() {
    let request = Request::post("/en/nowhere")
        .header(header::ACCEPT, "text/event-stream")
        .body(Body::empty())
        .expect("request");
    let (status, body) = get(pages(Router::new()), request).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, "");
}

#[tokio::test]
async fn a_page_carries_the_status_its_handler_gives_it() {
    let (status, body) = get(pages(Router::new()), page("/en/gone")).await;
    assert_eq!(status, StatusCode::GONE);
    assert!(body.contains(">This shop has closed.</p>"), "{body}");
}

#[tokio::test]
async fn a_nested_router_shows_its_page_for_a_path_under_it() {
    let app = Router::new().nest("/shop", pages(Router::new()));
    let (status, body) = get(app, page("/shop/ja/nowhere")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains("ページが見つかりません。"), "{body}");
}

hypersig::routes! {
    fn plain_pages(Router<()>);
    const PLAIN: Route<u32, Sort> = "/plain/{id}" => |(), (id, _sort): (u32, Sort)| async move {
        (id < 10).then(|| shop("plain"))
    };
}

#[tokio::test]
async fn without_a_page_for_them_a_404_is_empty_and_a_400_names_the_query() {
    let (status, body) = get(plain_pages(Router::new()), page("/plain/12?by=name")).await;
    assert_eq!((status, body.as_str()), (StatusCode::NOT_FOUND, ""));
    let (status, body) = get(plain_pages(Router::new()), page("/plain/x?by=name")).await;
    assert_eq!((status, body.as_str()), (StatusCode::NOT_FOUND, ""));
    let (status, body) = get(plain_pages(Router::new()), page("/plain/1?by=bogus")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(
        body.starts_with("Failed to deserialize query string"),
        "{body}"
    );
}

#[tokio::test]
async fn the_kind_of_a_request_follows_what_it_accepts() {
    let app: Router = Router::new().route(
        "/kind",
        axum::routing::get(|kind: RequestKind| async move { format!("{kind:?}") }),
    );
    let (_, body) = get(app.clone(), page("/kind")).await;
    assert_eq!(body, "Navigation");
    let request = Request::get("/kind")
        .header(header::ACCEPT, "text/event-stream")
        .body(Body::empty())
        .expect("request");
    let (_, body) = get(app, request).await;
    assert_eq!(body, "Runtime");
    let _ = (ORDER, PRODUCTS, GONE, PLAIN);
}
