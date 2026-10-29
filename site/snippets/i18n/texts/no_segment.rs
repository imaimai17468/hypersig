use hypersig_i18n::Locale;
use serde::{Deserialize, Serialize};

// No `#[derive(Segment)]`.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    Ja,
}

struct ByLang<T> {
    en: T,
    ja: T,
}

// error[E0277]: `Lang` does not list its variants
impl Locale for Lang {
    type Text<T> = ByLang<T>;

    fn pick<T>(self, text: ByLang<T>) -> T {
        match self {
            Self::En => text.en,
            Self::Ja => text.ja,
        }
    }
}
