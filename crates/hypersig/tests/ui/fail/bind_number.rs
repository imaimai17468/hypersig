#![allow(unused)]

use hypersig::html::{button, div, input, span};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: u64,
    name: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Other {
    flag: bool,
}

fn main() {
    let _ = input::<Counter>().bind(Counter::count());
}
