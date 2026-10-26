#![allow(unused)]

use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    price: f64,
    #[signals(computed)]
    total: f64,
}

fn main() {
    let cart = Cart { price: 2.5, total: 0.0 };
    let _ = Page::new("Cart", cart).computed(Cart::total().read(), Cart::price() * 2.0);
}
