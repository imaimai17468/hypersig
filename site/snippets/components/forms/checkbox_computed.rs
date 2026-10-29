use hypersig::Signals;
use hypersig_ui::checkbox;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Order {
    total: f64,
    #[signals(computed)]
    free_shipping: bool,
}

// A computed signal, which the page derives and a click cannot set.
let shipping = checkbox("Free shipping", Order::free_shipping());
# let _ = shipping;
