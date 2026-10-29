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
// Each translation puts the values where its language puts them.
const PAGE_OF: ByLang<fn(u32, u32) -> String> = ByLang {
    en: |page, total| format!("Page {page} of {total}"),
    ja: |page, total| format!("{total} ページ中 {page} ページ目"),
};

const REPLIED: ByLang<fn(&str, &str) -> String> = ByLang {
    en: |user, post| format!("{user} replied to “{post}”"),
    ja: |user, post| format!("{user} さんが「{post}」に返信しました"),
};

assert_eq!(Lang::En.pick(PAGE_OF)(2, 5), "Page 2 of 5");
assert_eq!(Lang::Ja.pick(PAGE_OF)(2, 5), "5 ページ中 2 ページ目");
assert_eq!(Lang::Ja.pick(REPLIED)("Ada", "Release notes"), "Ada さんが「Release notes」に返信しました");
