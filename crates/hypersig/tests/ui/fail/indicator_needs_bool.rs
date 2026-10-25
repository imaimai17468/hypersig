#![allow(unused)]
use hypersig::html::button;
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Form {
    saves: i32,
}

hypersig::routes! {
    fn actions(Router<()>);
    action SAVE: Action<Form> = post "/save" => |(), ()| async { Patches::<Form>::new() };
}

fn main() {
    let _ = button::<Form>().on_click(SAVE.call().indicator(Form::saves()));
}
