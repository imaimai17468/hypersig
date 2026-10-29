use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    price: f64,
    total: f64,
}

// error[E0277]: `Page::computed` takes a field marked `#[signals(computed)]`
let page = Page::new("Cart", Cart { price: 2.5, total: 0.0 })
    .computed(Cart::total(), Cart::price() * 2.0);
