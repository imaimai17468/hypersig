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
# use hypersig::Signals;
# #[derive(Signals, Serialize, Deserialize)]
# struct Profile {}
# async fn member((): (), _: (Lang, i32)) -> Option<hypersig::Page<Profile>> { None }
// `Lang` is a `Locale` enum with the variants `En` and `Ja`.
use axum::Router;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{HeaderMap, HeaderValue, Request, StatusCode, header};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use hypersig::Route;
use hypersig_i18n::negotiate;
use tower::ServiceExt as _;

hypersig::routes! {
    fn pages(Router<()>);
    const MEMBER: Route<(Lang, i32)> = "/{lang}/members/{id}" => member;
}

/// Sends `/members/7`, a link from before the site had languages, to the
/// member's page in the visitor's language.
async fn old_member(Path(id): Path<i32>, headers: HeaderMap) -> Response {
    let accept = headers
        .get(header::ACCEPT_LANGUAGE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    let lang = negotiate::<Lang>(accept);
    (
        [(header::VARY, HeaderValue::from_static("accept-language"))],
        Redirect::temporary(MEMBER.url((lang, id)).as_str()),
    )
        .into_response()
}

let app = pages(Router::new()).route("/members/{id}", get(old_member));

# tokio::runtime::Runtime::new().unwrap().block_on(async {
let request = Request::get("/members/7")
    .header(header::ACCEPT_LANGUAGE, "ja-JP,ja;q=0.9")
    .body(Body::empty())
    .unwrap();
let response = app.oneshot(request).await.unwrap();
assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
assert_eq!(response.headers()[header::LOCATION], "/ja/members/7");
# });
