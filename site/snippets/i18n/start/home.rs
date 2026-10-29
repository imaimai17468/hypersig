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
// `Lang` and `ByLang` are the ones declared above.
use axum::Router;
use hypersig::html::{h1, p};
use hypersig::{Page, Route, RouterExt, Signals};
use hypersig_i18n::LocaleRouterExt;

#[derive(Signals, Serialize, Deserialize)]
struct Home {}

hypersig::routes! {
    fn pages(Router<()>);
    // `/en` and `/ja`. Any other first segment answers 404.
    const HOME: Route<Lang> = "/{lang}" => home;
}

async fn home((): (), lang: Lang) -> Page<Home> {
    Page::new(lang.pick(ByLang { en: "Welcome", ja: "ようこそ" }), Home {})
        .lang(lang.tag())
        .child(h1().text(lang.pick(ByLang { en: "Welcome", ja: "ようこそ" })))
        .child(p().text(lang.pick(ByLang { en: "Hello.", ja: "こんにちは。" })))
}

// `GET /` redirects to `/en` or `/ja` by the `Accept-Language` header.
let app: Router = pages(Router::new())
    .redirect_to_locale(HOME)
    .hypersig_runtime();
# let _ = app;
# let html = tokio::runtime::Runtime::new().unwrap().block_on(home((), Lang::Ja)).render();
# assert!(html.starts_with(r#"<!doctype html><html lang="ja">"#), "{html}");
# assert!(html.contains("<title>ようこそ</title>"), "{html}");
# assert!(html.contains("<h1>ようこそ</h1><p>こんにちは。</p>"), "{html}");
# assert_eq!(HOME.url(Lang::Ja).as_str(), "/ja");
