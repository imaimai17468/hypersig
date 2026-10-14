#![allow(unused)]
use hypersig::html::button;
use hypersig::{Action, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Form {
    saves: i32,
}

const SAVE: Action<Form> = Action::post("/save");

fn main() {
    let _ = button::<Form>().on_click(SAVE.call().pending(Form::saves()));
}
