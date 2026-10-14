#![allow(unused)]
use hypersig::html::button;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Todo {
    title: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Todos {
    items: Vec<Todo>,
    draft: String,
}

fn main() {
    let _ = button::<Todos>().on_click(Todos::items().push(Todos::draft()));
}
