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
use hypersig::html::{El, a, attr, li, nav, ul};
use hypersig::{Page, Route, Signals};
use hypersig_i18n::locales;

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    name: String,
}

hypersig::routes! {
    fn pages(Router<()>);
    const MEMBER: Route<(Lang, i32)> = "/{lang}/members/{id}" => member;
}

/// A link to the member `id` in each language, the current one marked.
fn switcher(current: Lang, id: i32) -> El<Profile> {
    nav().child(ul().children(locales::<Lang>().iter().map(|&lang| {
        let tag = lang.tag();
        let link = a()
            .href(MEMBER.link_with((&lang, id)))
            .attr(attr::HREFLANG, tag.as_str())
            .attr(attr::LANG, tag.as_str())
            .text(lang.pick(ByLang { en: "English", ja: "日本語" }));
        let link = if lang == current { link.attr(attr::ARIA_CURRENT, "page") } else { link };
        li().child(link)
    })))
}

async fn member((): (), (lang, id): (Lang, i32)) -> Page<Profile> {
    let title = lang.pick(ByLang { en: "Member", ja: "メンバー" });
    Page::new(title, Profile { name: "Ada".to_owned() })
        .lang(lang.tag())
        .child(switcher(lang, id))
}

assert_eq!(
    switcher(Lang::Ja, 7).render(),
    concat!(
        r#"<nav><ul>"#,
        r#"<li><a href="/en/members/7" hreflang="en" lang="en">English</a></li>"#,
        r#"<li><a href="/ja/members/7" hreflang="ja" lang="ja" aria-current="page">日本語</a></li>"#,
        r#"</ul></nav>"#,
    ),
);
# let _ = pages;
