use hypersig::html::{El, input};
use hypersig::{NodeRef, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
}

fn search_field() -> El<Search> {
    // Inside a function body, where `self::FIELD` does not reach the constant.
    hypersig::node_ref!(const FIELD: NodeRef<Search>);
    input().node_ref(FIELD).bind(Search::query())
}
# fn main() {
#     let _ = search_field();
# }
