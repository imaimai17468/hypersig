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
use hypersig::{Origin, Page, Route, Signals};
use hypersig_i18n::LocalePage;

#[derive(Signals, Serialize, Deserialize)]
struct Post {}

hypersig::routes! {
    fn pages(Router<()>);
    const POST: Route<(Lang, u32)> = "/{lang}/posts/{id}" => post;
}

const SITE: Origin = Origin::new("https://example.com");

async fn post((): (), (lang, id): (Lang, u32)) -> Page<Post> {
    let title = lang.pick(ByLang { en: "Release notes", ja: "リリースノート" });
    Page::new(title, Post {})
        .lang(lang.tag())
        .alternates(SITE, |other: Lang| POST.url((other, id)))
}

# tokio::runtime::Runtime::new().unwrap().block_on(async {
let html = post((), (Lang::Ja, 3)).await.render();
assert!(html.contains(concat!(
    r#"<link rel="alternate" hreflang="en" href="https://example.com/en/posts/3">"#,
    r#"<link rel="alternate" hreflang="ja" href="https://example.com/ja/posts/3">"#,
    r#"<link rel="alternate" hreflang="x-default" href="https://example.com/en/posts/3">"#,
)));
# });
# let _ = pages;
