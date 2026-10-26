#![allow(unused)]

use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    #[signals(derived)]
    total: f64,
}

fn main() {}
