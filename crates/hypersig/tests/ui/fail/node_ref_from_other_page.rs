#![allow(unused)]
use hypersig::html::{button, input};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Login {
    name: String,
}

hypersig::node_ref!(const FIELD: NodeRef<Login>);

fn main() {
    let _ = button::<Search>().on_click(FIELD.focus());
}
