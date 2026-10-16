#![allow(unused)]

use hypersig::html::{Property, span};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

const FONT_SIZE: Property = Property::new("fontSize");

fn main() {
    let _ = span::<Counter>().bind_style(FONT_SIZE, Counter::count().to_text());
}
