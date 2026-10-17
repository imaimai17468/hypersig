use std::time::Duration;

use hypersig::html::{Event, Key, form, input};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Chat {
    draft: String,
    sent: i32,
    typing: bool,
}

let draft = Chat::draft();
let sent = Chat::sent();

let composer = form::<Chat>()
    // Runs after `preventDefault`, so the page does not reload.
    .on_submit(sent.set(sent + 1).then(draft.set("")))
    .child(
        input()
            // Two-way: typing writes `draft`, and setting `draft` rewrites the field.
            .bind(draft)
            .on_key(Key::Escape, draft.set(""))
            .on(Event::Focus, Chat::typing().set(true))
            .on_debounced(Event::Input, Duration::from_millis(500), Chat::typing().set(false)),
    );
# let _ = composer;
