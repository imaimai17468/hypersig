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
// `Lang` and `ByLang` are the ones the Overview declares.
use hypersig::{PluralCategory, Signals};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    count: i32,
}

let lang = Lang::En;
// error[E0308]: mismatched types: expected `IntlLocale`, found `Lang`
let label = Cart::count().plural(lang, |category| match category {
    PluralCategory::One => " item",
    _ => " items",
});
# let _ = label;
