use hypersig::html::p;
use hypersig::{IntlLocale, Page, PluralCategory, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    unread: i32,
}

let label = Inbox::unread().plural(IntlLocale::En, |category| match category {
    PluralCategory::One => "message",
    _ => "messages",
});

// panicked: the page sets `lang` to `ja` and formats its plurals and dates only in `en`, ...
let _ = Page::new("受信箱", Inbox { unread: 3 })
    .lang(IntlLocale::Ja)
    .child(p().bind_text(label))
    .render();
