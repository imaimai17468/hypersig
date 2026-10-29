# use hypersig::Segment;
# use hypersig_i18n::Locale;
# use serde::{Deserialize, Serialize};
# struct ByLang<T> { en: T, ja: T, pt_br: T }
# impl Locale for Lang {
#     type Text<T> = ByLang<T>;
#     fn pick<T>(self, text: ByLang<T>) -> T { match self { Self::En => text.en, Self::Ja => text.ja, Self::PtBr => text.pt_br } }
# }
use hypersig_i18n::negotiate;

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    Ja,
    #[serde(rename = "pt-BR")]
    PtBr,
}

assert_eq!(negotiate::<Lang>("ja"), Lang::Ja);
assert_eq!(negotiate::<Lang>("JA"), Lang::Ja);
assert_eq!(negotiate::<Lang>("ja-JP"), Lang::Ja);
assert_eq!(negotiate::<Lang>("pt-br"), Lang::PtBr);
assert_eq!(negotiate::<Lang>("pt"), Lang::PtBr);
assert_eq!(negotiate::<Lang>("pt-PT"), Lang::En);
assert_eq!(negotiate::<Lang>("ja-JP,ja;q=0.9,en;q=0.8"), Lang::Ja);
assert_eq!(negotiate::<Lang>("en;q=0.5, ja"), Lang::Ja);
assert_eq!(negotiate::<Lang>("en;q=0.8, ja;q=0.8"), Lang::En);
assert_eq!(negotiate::<Lang>("fr, ja;q=0.5"), Lang::Ja);
assert_eq!(negotiate::<Lang>("ja;q=0, pt"), Lang::PtBr);
assert_eq!(negotiate::<Lang>("ja;q=nope"), Lang::En);
assert_eq!(negotiate::<Lang>("ja, *"), Lang::Ja);
assert_eq!(negotiate::<Lang>("*, ja"), Lang::En);
assert_eq!(negotiate::<Lang>("fr"), Lang::En);
assert_eq!(negotiate::<Lang>(""), Lang::En);
