#![allow(unused)]
use hypersig::html::{each, li, ul};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Todo {
    title: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct User {
    name: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Todos {
    items: Vec<Todo>,
}

fn main() {
    let _ = ul::<Todos>().child(each(Todos::items(), |todo| li().bind_text(todo.at(User::name()))));
}
