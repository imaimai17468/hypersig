//! The headers that say which pages may show a page in a frame.

use axum::Router;
use axum::body::Body;
use axum::http::{HeaderMap, Request, StatusCode, header};
use hypersig::{Framing, Origin, Page, PageResponse, Route, Signals};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Blank {
    count: i32,
}

const PORTAL: Origin = Origin::new("https://portal.example");
const SHOP: Origin = Origin::new("https://shop.example:8443");

hypersig::routes! {
    fn pages(Router<()>);
    const PLAIN: Route = "/" => |(), ()| async { Page::new("Plain", Blank { count: 0 }) };
    const PORTALS: Route = "/portals" => |(), ()| async {
        Page::new("Portals", Blank { count: 0 }).framing(Framing::Origins(vec![PORTAL, SHOP]))
    };
    const ANYWHERE: Route = "/anywhere" => |(), ()| async {
        Page::new("Anywhere", Blank { count: 0 }).framing(Framing::Anywhere)
    };
    const MISSING: Route = "/missing" => |(), ()| async {
        PageResponse::with_status(StatusCode::NOT_FOUND, Page::new("Missing", Blank { count: 0 }))
    };
    const OWN_POLICY: Route = "/own-policy" => |(), ()| async {
        (
            [(header::CONTENT_SECURITY_POLICY, "default-src 'self'")],
            Page::new("Own policy", Blank { count: 0 }),
        )
    };
    const OWN_POLICY_PORTALS: Route = "/own-policy-portals" => |(), ()| async {
        (
            [(header::CONTENT_SECURITY_POLICY, "default-src 'self'")],
            Page::new("Own policy", Blank { count: 0 }).framing(Framing::Origins(vec![PORTAL])),
        )
    };
    const OWN_FRAME_OPTIONS: Route = "/own-frame-options" => |(), ()| async {
        (
            [(header::X_FRAME_OPTIONS, "DENY")],
            Page::new("Own frame options", Blank { count: 0 }),
        )
    };
}

async fn headers(path: &str) -> HeaderMap {
    let response = pages(Router::new())
        .oneshot(Request::get(path).body(Body::empty()).expect("request"))
        .await
        .expect("the router is infallible");
    response.headers().clone()
}

fn value(headers: &HeaderMap, name: header::HeaderName) -> Option<&str> {
    headers
        .get(name)
        .map(|value| value.to_str().expect("ASCII"))
}

#[tokio::test]
async fn a_page_refuses_frames_from_other_sites() {
    for path in ["/", "/missing"] {
        let headers = headers(path).await;
        assert_eq!(
            value(&headers, header::X_FRAME_OPTIONS),
            Some("SAMEORIGIN"),
            "{path}"
        );
        assert_eq!(
            value(&headers, header::CONTENT_SECURITY_POLICY),
            Some("frame-ancestors 'self'"),
            "{path}"
        );
    }
}

#[tokio::test]
async fn a_page_framed_by_origins_names_them_in_frame_ancestors_alone() {
    let headers = headers("/portals").await;
    assert_eq!(value(&headers, header::X_FRAME_OPTIONS), None);
    assert_eq!(
        value(&headers, header::CONTENT_SECURITY_POLICY),
        Some("frame-ancestors 'self' https://portal.example https://shop.example:8443")
    );
}

#[tokio::test]
async fn a_page_framed_anywhere_sends_neither_header() {
    let headers = headers("/anywhere").await;
    assert_eq!(value(&headers, header::X_FRAME_OPTIONS), None);
    assert_eq!(value(&headers, header::CONTENT_SECURITY_POLICY), None);
}

fn policies(headers: &HeaderMap) -> Vec<&str> {
    headers
        .get_all(header::CONTENT_SECURITY_POLICY)
        .iter()
        .map(|value| value.to_str().expect("ASCII"))
        .collect()
}

#[tokio::test]
async fn a_policy_the_handler_sends_goes_out_beside_frame_ancestors() {
    let headers = headers("/own-policy").await;
    assert_eq!(
        policies(&headers),
        ["default-src 'self'", "frame-ancestors 'self'"]
    );
    assert_eq!(value(&headers, header::X_FRAME_OPTIONS), Some("SAMEORIGIN"));
}

#[tokio::test]
async fn a_page_framed_by_origins_keeps_frame_ancestors_beside_the_handler_s_policy() {
    let headers = headers("/own-policy-portals").await;
    assert_eq!(value(&headers, header::X_FRAME_OPTIONS), None);
    assert_eq!(
        policies(&headers),
        [
            "default-src 'self'",
            "frame-ancestors 'self' https://portal.example"
        ]
    );
}

#[tokio::test]
async fn an_x_frame_options_the_handler_sends_is_kept() {
    let headers = headers("/own-frame-options").await;
    assert_eq!(headers.get_all(header::X_FRAME_OPTIONS).iter().count(), 1);
    assert_eq!(value(&headers, header::X_FRAME_OPTIONS), Some("DENY"));
}
