#![allow(unused)]

use hypersig::html::{Class, Stylesheet, span};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

const STYLE: Stylesheet = Stylesheet::new(".big { color: crimson }");
const BIG: Class = STYLE.class("bigg");

fn main() {
    let _ = span::<Counter>().class(BIG);
}
