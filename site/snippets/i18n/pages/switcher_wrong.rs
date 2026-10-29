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
use hypersig::html::a;

hypersig::routes! {
    fn pages(Router<()>);
    const MEMBER: Route<(Lang, i32)> = "/{lang}/members/{id}" => member;
}

// error[E0277]: `&Lang` is not an expression of type `(Lang, i32)` on the page
// whose signals are `_`
let link = a::<Profile>().href(MEMBER.link_with(&Lang::Ja)).text("日本語");
# let _ = (pages, link);
