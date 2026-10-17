use hypersig::html::{button, span};
use hypersig::{Page, RUNTIME_JS, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

let count = Counter::count();
let html = Page::new("Counter", Counter { count: 0 })
    // Load the runtime from a file next to the page.
    .runtime_src("./hypersig.js")
    .child(button().on_click(count.set(count + 1)).text("+1"))
    .child(span().bind_text(count.to_text()))
    .render();

// Write `html` to index.html and RUNTIME_JS to hypersig.js, then serve both.
# assert!(html.contains(r#"src="./hypersig.js""#));
# assert!(!RUNTIME_JS.is_empty());
