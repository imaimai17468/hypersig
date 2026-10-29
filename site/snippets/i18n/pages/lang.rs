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
// `Lang` is a `Locale` enum with the variants `En` and `Ja`.
use hypersig::{IntlLocale, LanguageTag, Page, Signals};

#[derive(Signals, Serialize, Deserialize)]
struct Note {}

let html = Page::new("ノート", Note {}).lang(Lang::Ja.tag()).render();
assert!(html.starts_with(r#"<!doctype html><html lang="ja">"#));

// A tag written in the program, checked when it compiles.
const PT_BR: LanguageTag = LanguageTag::new("pt-BR");
let html = Page::new("Nota", Note {}).lang(PT_BR).render();
assert!(html.starts_with(r#"<!doctype html><html lang="pt-BR">"#));

// The locale the page writes its numbers and dates in.
let html = Page::new("Note", Note {}).lang(IntlLocale::En).render();
assert!(html.starts_with(r#"<!doctype html><html lang="en">"#));
