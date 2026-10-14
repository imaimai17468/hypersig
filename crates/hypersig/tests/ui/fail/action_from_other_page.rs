#![allow(unused)]

use hypersig::html::{button, div, input, span};
use hypersig::{Action, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
    name: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Other {
    flag: bool,
}

const TOGGLE: Action<Other> = Action::post("/toggle");

fn main() {
    let _ = button::<Counter>().on_click(TOGGLE.call());
}
