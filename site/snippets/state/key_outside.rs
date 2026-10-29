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

// error[E0599]: no method named `key` found for struct `El<S>` in the current scope
let list = ul::<Todos>().child(each(Todos::items(), |todo| {
    li().child(span().key(todo.at(Todo::id())))
}));
