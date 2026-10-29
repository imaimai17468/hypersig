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

// error[E0277]: `Sig<Todos, String>` is not an expression of type `Todo` on the page whose signals are `Todos`
let add = button::<Todos>().on_click(Todos::items().push(Todos::draft()));
