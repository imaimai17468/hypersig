#![allow(unused)]
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

fn main() {
    let _ = ul::<Todos>().child(each(Todos::items(), |todo| {
        // The server cannot patch "whichever item the browser is rendering".
        let _: Patches<Todos> = todo.at(Todo::done()).patch(true).into();
        li()
    }));
}
