use hypersig::html::{p, span};
use hypersig::{Page, Signals};
use hypersig_ui::kbd;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Help {}

let html = Page::new("Help", Help {})
    // One `kbd` per key, written as a word a screen reader reads correctly.
    .child(
        span()
            .child(kbd().text("Ctrl"))
            .child(kbd().text("Shift"))
            .child(kbd().text("P")),
    )
    .child(p().text("Press ").child(kbd().text("Esc")).text(" to close the dialog."))
    .render();
# assert!(html.contains(">Esc</kbd>"));
