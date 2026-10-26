// A language written as a string could be any text, so `lang` takes a `LanguageTag` or an `IntlLocale`.
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Note {}

fn main() {
    let _ = Page::new("Note", Note {}).lang("en_US");
}
