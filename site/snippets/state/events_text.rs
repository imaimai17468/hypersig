use hypersig::html::input;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
    keystrokes: i32,
    committed: String,
}

let query = Search::query();
let keystrokes = Search::keystrokes();

let field = input::<Search>()
    .bind(query)
    // Each edit of the text.
    .on_input(keystrokes.set(keystrokes + 1))
    // Focus leaves the field after its text changed.
    .on_change(Search::committed().set(query));
# let html = Page::new("Search", Search { query: String::new(), keystrokes: 0, committed: String::new() }).child(field).render();
# let _ = html;
