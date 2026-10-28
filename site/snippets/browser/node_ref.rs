use hypersig::html::{button, div, input};
use hypersig::{NodeRef, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
}

// At module level, outside every function.
hypersig::node_ref!(const FIELD: NodeRef<Search>);
# fn main() {

let search = div::<Search>()
    .child(input().node_ref(FIELD).bind(Search::query()))
    .child(button().on_click(FIELD.focus()).text("Search"));
# let _ = search.render();
# }
