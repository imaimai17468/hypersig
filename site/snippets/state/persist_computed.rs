use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    price: f64,
    #[signals(computed)]
    total: f64,
}

// error[E0308]: mismatched types: expected `Sig<Cart, _>`, found `Sig<Cart, f64, Computed>`
let page = Page::new("Cart", Cart { price: 2.5, total: 0.0 }).persist(Cart::total());
