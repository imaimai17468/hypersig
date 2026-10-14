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

#[derive(Serialize)]
struct Id(u32);

const REMOVE: Action<Counter, Id> = Action::post("/remove");

fn main() {
    let _ = REMOVE.call();
}
