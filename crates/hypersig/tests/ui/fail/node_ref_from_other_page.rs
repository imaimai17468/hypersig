#![allow(unused)]
use hypersig::html::{button, input};
use hypersig::{NodeRef, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Login {
    name: String,
}

const FIELD: NodeRef<Login> = NodeRef::new("name");

fn main() {
    let _ = button::<Search>().on_click(FIELD.focus());
}
