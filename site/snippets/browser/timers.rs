use std::time::Duration;

use hypersig::html::{div, p, span};
use hypersig::{Page, Signals, Stmt};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Clock {
    running: bool,
    tenths: i32,
    hint: bool,
}

let (running, tenths, hint) = (Clock::running(), Clock::tenths(), Clock::hint());
let signals = Clock { running: true, tenths: 0, hint: false };
let page = Page::new("Clock", signals).child(
    div()
        // Every 100 milliseconds while the element is in the document.
        .on_interval(Duration::from_millis(100), Stmt::when(running, tenths.set(tenths + 1)))
        // Once, 3 seconds after the element is added.
        .on_timeout(Duration::from_secs(3), hint.set(true))
        .child(span().bind_text(tenths.to_text()))
        .child(p().show(hint).text("Tenths of a second since the page loaded")),
);
# assert!(page.render().contains(">0</span>"));
