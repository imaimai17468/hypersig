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
use axum::http::{Request, StatusCode};
use hypersig::html::{h1, p};
use hypersig::{Page, Route, Signals};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    name: String,
}

hypersig::routes! {
    fn pages(Router<()>);
    const MEMBER: Route<(Lang, i32)> = "/{lang}/members/{id}" => member;
}

async fn member((): (), (lang, id): (Lang, i32)) -> Option<Page<Profile>> {
    let name = match id {
        7 => "Ada",
        _ => return None,
    };
    let title = lang.pick(ByLang { en: "Member", ja: "メンバー" });
    let joined = lang.pick(ByLang { en: "Joined in 2024.", ja: "2024 年に参加しました。" });
    Some(
        Page::new(title, Profile { name: name.to_owned() })
            .lang(lang.tag())
            .child(h1().text(name))
            .child(p().text(joined)),
    )
}

async fn status(path: &str) -> StatusCode {
    let request = Request::get(path).body(Body::empty()).unwrap();
    pages(Router::new()).oneshot(request).await.unwrap().status()
}

# tokio::runtime::Runtime::new().unwrap().block_on(async {
assert_eq!(status(MEMBER.url((Lang::Ja, 7)).as_str()).await, StatusCode::OK);
assert_eq!(status("/fr/members/7").await, StatusCode::NOT_FOUND);
assert_eq!(status("/EN/members/7").await, StatusCode::NOT_FOUND);
assert_eq!(status("/ja/members/8").await, StatusCode::NOT_FOUND);
# });
