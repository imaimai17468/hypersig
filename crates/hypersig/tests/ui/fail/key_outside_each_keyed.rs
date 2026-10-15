#![allow(unused)]
use hypersig::html::{each, li, span, ul};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Todo {
    id: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Todos {
    items: Vec<Todo>,
}

fn main() {
    // A key belongs to the root of an `each_keyed` template.
    let _ = ul::<Todos>().child(each(Todos::items(), |todo| {
        li().child(span().key(todo.at(Todo::id())))
    }));
}
