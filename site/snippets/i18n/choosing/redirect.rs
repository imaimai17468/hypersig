# use hypersig::Segment;
# use hypersig_i18n::Locale;
# use serde::{Deserialize, Serialize};
# #[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
# #[serde(rename_all = "lowercase")]
# enum Lang { En, Ja }
# struct ByLang<T> { en: T, ja: T }
# impl Locale for Lang {
#     type Text<T> = ByLang<T>;
#     fn pick<T>(self, text: ByLang<T>) -> T { match self { Self::En => text.en, Self::Ja => text.ja } }
# }
// `Lang` is a `Locale` enum with the variants `En` and `Ja`, and `ByLang` its text struct.
use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use hypersig::html::h1;
use hypersig::{Page, Route, Signals};
use hypersig_i18n::LocaleRouterExt;
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Home {}

hypersig::routes! {
    fn pages(Router<()>);
    const HOME: Route<Lang> = "/{lang}" => home;
}

async fn home((): (), lang: Lang) -> Page<Home> {
    let title = lang.pick(ByLang { en: "Welcome", ja: "ようこそ" });
    Page::new(title, Home {}).lang(lang.tag()).child(h1().text(title))
}

fn app() -> Router {
    pages(Router::new()).redirect_to_locale(HOME)
}

# tokio::runtime::Runtime::new().unwrap().block_on(async {
let request = Request::get("/")
    .header(header::ACCEPT_LANGUAGE, "ja-JP,ja;q=0.9,en;q=0.8")
    .body(Body::empty())
    .unwrap();
let response = app().oneshot(request).await.unwrap();
assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
assert_eq!(response.headers()[header::LOCATION], "/ja");
assert_eq!(response.headers()[header::VARY], "accept-language");

let request = Request::get("/").body(Body::empty()).unwrap();
let response = app().oneshot(request).await.unwrap();
assert_eq!(response.headers()[header::LOCATION], "/en");
# });
