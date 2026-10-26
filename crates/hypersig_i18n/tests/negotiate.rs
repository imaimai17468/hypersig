//! Choosing a locale from `Accept-Language`, and the redirect from `/`.

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use hypersig::{Page, Route, Segment, Signals};
use hypersig_i18n::{Locale, LocaleRouterExt, locales, negotiate};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
enum Lang {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "ja")]
    Ja,
}

struct ByLang<T> {
    en: T,
    en_gb: T,
    ja: T,
}

impl Locale for Lang {
    type Text<T> = ByLang<T>;

    fn pick<T>(self, text: ByLang<T>) -> T {
        match self {
            Self::En => text.en,
            Self::EnGb => text.en_gb,
            Self::Ja => text.ja,
        }
    }
}

#[derive(Signals, Serialize, Deserialize)]
struct Home {
    visits: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    const HOME: Route<Lang> = "/{lang}" => |(), lang: Lang| async move {
        Page::new(lang.pick(ByLang { en: "Color", en_gb: "Colour", ja: "色" }), Home { visits: 0 })
            .lang(lang.tag())
    };
}

#[test]
fn every_variant_is_a_locale_in_declaration_order() {
    assert_eq!(locales::<Lang>(), [Lang::En, Lang::EnGb, Lang::Ja]);
}

#[test]
fn prefers_the_highest_weight_then_the_written_order() {
    assert_eq!(negotiate::<Lang>("en;q=0.5, ja"), Lang::Ja);
    assert_eq!(negotiate::<Lang>("ja, en"), Lang::Ja);
    assert_eq!(negotiate::<Lang>("ja;q=0, en-gb"), Lang::EnGb);
}

#[test]
fn matches_a_range_by_its_prefix_and_a_tag_by_the_range() {
    assert_eq!(negotiate::<Lang>("ja-JP"), Lang::Ja);
    assert_eq!(negotiate::<Lang>("en-AU"), Lang::En);
    assert_eq!(negotiate::<Lang>("EN-gb"), Lang::EnGb);
}

#[test]
fn falls_back_to_the_first_locale() {
    assert_eq!(negotiate::<Lang>(""), Lang::En);
    assert_eq!(negotiate::<Lang>("fr, de;q=0.8"), Lang::En);
    assert_eq!(negotiate::<Lang>("*"), Lang::En);
    assert_eq!(negotiate::<Lang>("ja;q=nope"), Lang::En);
}

#[tokio::test]
async fn sends_the_root_to_the_negotiated_page() {
    let app = pages(Router::new()).redirect_to_locale(HOME);
    let response = app
        .clone()
        .oneshot(
            Request::get("/")
                .header("accept-language", "ja-JP,ja;q=0.9")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("the router is infallible");
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(response.headers()["location"], "/ja");
    assert_eq!(response.headers()["vary"], "accept-language");

    let page = app
        .oneshot(Request::get("/en-GB").body(Body::empty()).expect("request"))
        .await
        .expect("the router is infallible");
    let body = http_body_util::BodyExt::collect(page.into_body())
        .await
        .expect("body")
        .to_bytes();
    let body = String::from_utf8(body.to_vec()).expect("utf-8");
    assert!(
        body.starts_with(r#"<!doctype html><html lang="en-GB">"#),
        "{body}"
    );
    assert!(body.contains("<title>Colour</title>"), "{body}");
}
