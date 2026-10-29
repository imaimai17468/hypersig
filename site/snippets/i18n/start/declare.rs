use hypersig::Segment;
use hypersig_i18n::Locale;
use serde::{Deserialize, Serialize};

// The segment of each language is its BCP 47 tag, here `en` and `ja`.
#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    Ja,
}

/// One value per language.
struct ByLang<T> {
    en: T,
    ja: T,
}

impl Locale for Lang {
    type Text<T> = ByLang<T>;

    fn pick<T>(self, text: ByLang<T>) -> T {
        match self {
            Self::En => text.en,
            Self::Ja => text.ja,
        }
    }
}

assert_eq!(Lang::Ja.pick(ByLang { en: "Hello", ja: "こんにちは" }), "こんにちは");
