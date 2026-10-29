use hypersig::html::input;
use hypersig::{NodeRef, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    coupon: String,
}

hypersig::node_ref!(const FIELD: NodeRef<Search>);
# fn main() {

// `FIELD` names an element of the page whose signals are `Search`.
let coupon = input::<Cart>().node_ref(FIELD).bind(Cart::coupon());
# let _ = coupon;
# }
