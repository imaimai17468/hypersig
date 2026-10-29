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
let items = ByLang {
    en: |n: u32| format!("{n} items"),
    // error[E0308]: mismatched types: no two closures, even if identical, have the same type
    ja: |n: u32| format!("{n} 件"),
};
# let _ = Lang::En.pick(items)(1);
