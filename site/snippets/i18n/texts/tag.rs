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
use hypersig_i18n::{locales, negotiate};

// Every variant, in declaration order.
assert_eq!(locales::<Lang>(), [Lang::En, Lang::Ja]);
// The tag is the string serde writes for the variant.
assert_eq!(Lang::Ja.tag().as_str(), "ja");
// The first variant is the default.
assert_eq!(negotiate::<Lang>("fr"), Lang::En);
