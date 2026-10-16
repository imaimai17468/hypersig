#![allow(unused)]

use hypersig::html::{Class, span};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

const BIG: Class = Class::new("big red");

fn main() {
    let _ = span::<Counter>().class(BIG);
}
