# use hypersig::html::input;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    price: f64,
    quantity: i32,
    #[signals(computed)]
    total: f64,
}

// error[E0277]: a signal marked `Computed` cannot be written
let field = input::<Cart>().bind(Cart::total());
