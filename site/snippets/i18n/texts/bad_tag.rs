use hypersig::Segment;
use hypersig_i18n::Locale;
use serde::{Deserialize, Serialize};

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
enum Lang {
    // A BCP 47 tag separates its subtags with `-`, not `_`.
    #[serde(rename = "en_US")]
    EnUs,
    #[serde(rename = "ja")]
    Ja,
}

struct ByLang<T> {
    en_us: T,
    ja: T,
}

impl Locale for Lang {
    type Text<T> = ByLang<T>;

    fn pick<T>(self, text: ByLang<T>) -> T {
        match self {
            Self::EnUs => text.en_us,
            Self::Ja => text.ja,
        }
    }
}

// panicked: a locale's segment is its language tag:
// `en_US` is not a BCP 47 language tag
let _ = Lang::EnUs.tag();
