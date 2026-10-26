#![allow(unused)]

use hypersig::html::input;
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
    let cart = Cart { price: 2.5, quantity: 3, lines: Vec::new(), total: 0.0, totals: Vec::new() };
    let _ = Page::new("Cart", cart).computed(Cart::price(), Cart::price() * 2.0);
}
