use hypersig::html::{div, input};
use hypersig::{NodeRef, Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
    filter: String,
}

hypersig::node_ref!(const FIELD: NodeRef<Search>);
# fn main() {

let signals = Search { query: String::new(), filter: String::new() };
let page = Page::new("Search", signals).child(
    div()
        .child(input().node_ref(FIELD).bind(Search::query()))
        // A second element with the same name.
        .child(input().node_ref(FIELD).bind(Search::filter())),
);
// Panics: "two elements carry the node ref `...::FIELD` where the runtime can reach both".
let _ = page.render();
# }
