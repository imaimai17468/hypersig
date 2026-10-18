use hypersig::Signals;
use hypersig_ui::NumberField;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    note: String,
}

fn main() {
    // A number field holds an `i32` or an `f64`, so a text signal cannot be its value.
    let _ = NumberField::new("Quantity", Cart::note()).render();
}
