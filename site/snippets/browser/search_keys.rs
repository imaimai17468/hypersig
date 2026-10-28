use hypersig::html::{Key, input};
use hypersig::{NodeRef, Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Docs {
    query: String,
}

hypersig::node_ref!(const SEARCH: NodeRef<Docs>);
# fn main() {

let query = Docs::query();
let page = Page::new("Docs", Docs { query: String::new() })
    // `/` anywhere on the page, outside a text field, moves focus to the search field.
    .on_key(Key::Char('/'), SEARCH.focus())
    .child(
        input()
            .node_ref(SEARCH)
            .bind(query)
            // Escape in the field clears it and gives focus back to the page.
            .on_key(Key::Escape, query.set("").then(SEARCH.blur())),
    );
# let _ = page.render();
# }
