use hypersig::Segment;
use hypersig_i18n::Locale;

// No `Serialize`.
#[derive(Segment, Clone, Copy, PartialEq, Eq, Debug)]
enum Lang {
    En,
    Ja,
}

struct ByLang<T> {
    en: T,
    ja: T,
}

// error[E0277]: the trait bound `Lang: serde::Serialize` is not satisfied
impl Locale for Lang {
    type Text<T> = ByLang<T>;

    fn pick<T>(self, text: ByLang<T>) -> T {
        match self {
            Self::En => text.en,
            Self::Ja => text.ja,
        }
    }
}
