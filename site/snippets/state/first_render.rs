use hypersig::html::{button, span};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

let count = Counter::count();
let html = Page::new("Counter", Counter { count: 2 })
    .child(button().on_click(count.set(count + 1)).text("+1"))
    .child(span().bind_text(count.to_text()))
    .render();

// The server evaluated the binding with the initial value.
assert!(html.contains(">2</span>"));
