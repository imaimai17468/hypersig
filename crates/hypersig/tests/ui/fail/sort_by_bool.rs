#![allow(unused)]
use hypersig::{Signals, Stmt};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Todo {
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Todos {
    items: Vec<Todo>,
}

fn main() {
    // The runtime orders only numbers and strings.
    let _: Stmt<Todos> = Todos::items().sort_by(|todo| todo.at(Todo::done()));
}
