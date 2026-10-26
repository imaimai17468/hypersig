#![allow(unused)]

use hypersig::html::{button, div, input, span};
use hypersig::{Action, Patches, Signals};
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

#[derive(Serialize, Deserialize)]
struct Id(u32);

hypersig::routes! {
    fn actions(Router<()>);
    action REMOVE: Action<Counter, Id> = post "/remove" => |(), _: Id| async { Patches::<Counter>::new() };
}

fn main() {
    let _ = REMOVE.call();
}
