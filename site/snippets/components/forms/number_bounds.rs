use hypersig::Signals;
use hypersig_ui::NumberField;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Order {
    quantity: i32,
}

// Panics: "a number field's maximum is not below its minimum, but 1 is below 10".
let quantity = NumberField::new("Quantity", Order::quantity()).min(10).max(1);
# let _ = quantity;
