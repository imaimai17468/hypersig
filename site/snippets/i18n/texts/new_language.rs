use hypersig::Segment;
use hypersig_i18n::Locale;
use serde::{Deserialize, Serialize};

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    Ja,
    Fr, // added
}

struct ByLang<T> {
    en: T,
    ja: T,
    fr: T, // added
}

impl Locale for Lang {
    type Text<T> = ByLang<T>;

    fn pick<T>(self, text: ByLang<T>) -> T {
        // error[E0004]: non-exhaustive patterns: `Lang::Fr` not covered
        match self {
            Self::En => text.en,
            Self::Ja => text.ja,
        }
    }
}

// error[E0063]: missing field `fr` in initializer of `ByLang<_>`
const SETTINGS: ByLang<&str> = ByLang { en: "Settings", ja: "設定" };
# let _ = Lang::En.pick(SETTINGS);
