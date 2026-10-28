use hypersig::html::{Id, Role, div, input};
use hypersig::{NodeRef, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
    // The id of the option the arrow keys reached.
    active: String,
}

hypersig::node_ref!(const RESULTS: NodeRef<Search>);
const RUST: Id = Id::new("option-rust");
const ZIG: Id = Id::new("option-zig");
# fn main() {

let field = input::<Search>()
    .role(Role::Combobox)
    .bind(Search::query())
    .active_descendant(RESULTS, Search::active());
let results = div::<Search>()
    .role(Role::ListBox)
    .node_ref(RESULTS)
    .child(div().role(Role::Option).id(RUST).text("Rust"))
    .child(div().role(Role::Option).id(ZIG).text("Zig"));
# let _ = (field.render(), results.render());
# }
