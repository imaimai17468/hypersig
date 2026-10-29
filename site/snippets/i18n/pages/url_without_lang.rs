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
# use hypersig::{Page, Signals};
# #[derive(Signals, Serialize, Deserialize)]
# struct Profile {}
# async fn member((): (), _: (Lang, i32)) -> Option<Page<Profile>> { None }
use hypersig::Route;

hypersig::routes! {
    fn pages(Router<()>);
    const MEMBER: Route<(Lang, i32)> = "/{lang}/members/{id}" => member;
}

// error[E0308]: mismatched types: expected `(Lang, i32)`, found integer
let url = MEMBER.url(7);
# let _ = (pages, url);
