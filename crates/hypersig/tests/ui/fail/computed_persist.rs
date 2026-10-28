#![allow(unused)]

use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    price: f64,
    quantity: i32,
    lines: Vec<f64>,
    #[signals(computed)]
    total: f64,
    #[signals(computed)]
    totals: Vec<f64>,
}

fn main() {
    let _ = Page::new("Cart", Cart { price: 0.0, quantity: 0, lines: Vec::new(), total: 0.0, totals: Vec::new() })
        .persist(Cart::total());
}
