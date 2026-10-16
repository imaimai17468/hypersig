#![allow(unused)]

use hypersig::html::span;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

fn main() {
    let _ = span::<Counter>().class("big");
}
