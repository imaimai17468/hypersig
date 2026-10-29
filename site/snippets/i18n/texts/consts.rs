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
// Texts shared by several pages, as constants.
const SAVE: ByLang<&str> = ByLang { en: "Save", ja: "保存" };
const CANCEL: ByLang<&str> = ByLang { en: "Cancel", ja: "キャンセル" };

// A text used in one place, written where it is used.
fn heading(lang: Lang) -> &'static str {
    lang.pick(ByLang { en: "Settings", ja: "設定" })
}

// Each use of a constant is a new value, so `pick` can take it by value again.
assert_eq!(Lang::Ja.pick(SAVE), "保存");
assert_eq!(Lang::En.pick(SAVE), "Save");
assert_eq!(Lang::Ja.pick(CANCEL), "キャンセル");
assert_eq!(heading(Lang::Ja), "設定");
