use hypersig::Segment;
use hypersig_i18n::Locale;
use serde::{Deserialize, Serialize};

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    Ja,
}

struct ByLang<T> {
    en: T,
    ja: T,
}

impl Locale for Lang {
    type Text<T> = ByLang<T>;

    fn pick<T>(self, text: ByLang<T>) -> T {
        // error[E0004]: non-exhaustive patterns: `Lang::Ja` not covered
        match self {
            Self::En => text.en,
        }
    }
}
