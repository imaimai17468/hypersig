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
use hypersig::{Page, Route, Signals};

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
    Some(Page::new(title, Profile { name: name.to_owned() }))
}

assert_eq!(MEMBER.url((Lang::En, 7)).as_str(), "/en/members/7");
assert_eq!(MEMBER.url((Lang::Ja, 7)).as_str(), "/ja/members/7");
# let _ = pages;
