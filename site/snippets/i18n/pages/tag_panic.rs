# use hypersig::Segment;
# use serde::{Deserialize, Serialize};
use hypersig_i18n::Locale;

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
enum Lang {
    #[serde(rename = "en_US")]
    EnUs,
}

impl Locale for Lang {
    type Text<T> = T;

    fn pick<T>(self, text: T) -> T {
        text
    }
}

// Panics: a locale's segment is its language tag: `en_US` is not a BCP 47
// language tag
let tag = Lang::EnUs.tag();
# let _ = tag;
