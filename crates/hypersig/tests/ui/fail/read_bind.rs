#![allow(unused)]

use hypersig::Signals;
use hypersig::html::input;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    price: f64,
}

fn main() {
    let _ = input::<Cart>().bind(Cart::price().read());
}
