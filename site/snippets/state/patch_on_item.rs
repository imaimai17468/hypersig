use hypersig::html::{each, li, ul};
use hypersig::{Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Todo {
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Todos {
    items: Vec<Todo>,
}

let list = ul::<Todos>().child(each(Todos::items(), |todo| {
    // error[E0599]: no method named `patch` found for struct `Sig<Todos, bool, hypersig::Local>`
    let _: Patches<Todos> = todo.at(Todo::done()).patch(true).into();
    li()
}));
