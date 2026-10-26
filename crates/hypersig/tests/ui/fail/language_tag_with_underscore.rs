// `lang` and `hreflang` take a BCP 47 tag, whose subtags are joined by `-`, so `en_US` is no language.
use hypersig::LanguageTag;

const EN_US: LanguageTag = LanguageTag::new("en_US");

fn main() {
    let _ = EN_US;
}
