use hypersig::html::{div, input, p};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
    long: bool,
    edits: i32,
}

let (query, long, edits) = (Search::query(), Search::long(), Search::edits());
let signals = Search { query: String::new(), long: false, edits: 0 };
let page = Page::new("Search", signals).child(
    div()
        // Reads `query`, so it runs again whenever `query` changes.
        .effect(long.set(query.char_count().gt(20)))
        // Does not read `query`, so it names the signal to watch.
        .on_signal_change(query, edits.set(edits + 1))
        .child(input().bind(query))
        .child(p().show(long).text("Try fewer words")),
);
# let _ = page.render();
