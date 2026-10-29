use hypersig::html::{Role, input};
use hypersig::{NodeRef, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
    // The position of the option, where the method takes its id.
    active: i32,
}

hypersig::node_ref!(const RESULTS: NodeRef<Search>);
# fn main() {

let field = input::<Search>()
    .role(Role::Combobox)
    .bind(Search::query())
    .active_descendant(RESULTS, Search::active());
# let _ = field;
# }
