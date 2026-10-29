use hypersig::Signals;
use hypersig_ui::NumberField;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    quantity: String,
}

// A text signal bound to a number field.
let quantity = NumberField::new("Quantity", Cart::quantity());
# let _ = quantity;
