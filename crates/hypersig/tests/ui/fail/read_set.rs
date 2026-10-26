#![allow(unused)]

use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    price: f64,
}

fn main() {
    let _ = Cart::price().read().set(0.0);
}
