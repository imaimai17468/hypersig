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

hypersig::routes! {
    fn actions(Router<()>);
    action TOGGLE: Action<Other> = post "/toggle" => |(), ()| async { Patches::<Other>::new() };
}

fn main() {
    let _ = button::<Counter>().on_click(TOGGLE.call());
}
