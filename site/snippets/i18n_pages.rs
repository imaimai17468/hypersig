# use hypersig::Segment;
# use hypersig_i18n::Locale;
# use serde::{Deserialize, Serialize};
# #[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
# #[serde(rename_all = "lowercase")]
# enum Lang { En, Ja }
# struct ByLang<T> { en: T, ja: T }
# impl Locale for Lang {
#     const ALL: &'static [Self] = &[Self::En, Self::Ja];
#     type Text<T> = ByLang<T>;
#     fn pick<T>(self, text: ByLang<T>) -> T { match self { Self::En => text.en, Self::Ja => text.ja } }
# }
// `Lang` and `ByLang` are the ones the Overview declares.
# use hypersig::Signals;
use hypersig::html::a;
use hypersig::{Page, Route};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    name: String,
}

hypersig::routes! {
    fn pages(Router<()>);
    // The language is the first segment of every page.
    const MEMBER: Route<(Lang, i32)> = "/{lang}/members/{id}" => member;
}

async fn member((): (), (lang, id): (Lang, i32)) -> Page<Profile> {
    let other = if lang == Lang::En { Lang::Ja } else { Lang::En };
    Page::new(lang.pick(ByLang { en: "Member", ja: "メンバー" }), Profile { name: "Ada".to_owned() })
        .lang(lang.tag())
        // The same page in the other language is the same route with another locale.
        .child(a().href(MEMBER.link_with((&other, id))).text(other.pick(ByLang { en: "English", ja: "日本語" })))
}

assert_eq!(MEMBER.url((Lang::Ja, 7)).as_str(), "/ja/members/7");
# let _ = pages;
