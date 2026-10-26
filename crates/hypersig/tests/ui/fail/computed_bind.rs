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
    let _ = input::<Cart>().bind(Cart::total());
}
