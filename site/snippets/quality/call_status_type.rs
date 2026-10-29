use hypersig::html::button;
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    problem: String,
    failure: i32,
}

hypersig::routes! {
    fn actions(Router<()>);
    action SAVE: Action<Notes> = post "/save" => save;
}
# async fn save((): (), (): ()) -> Patches<Notes> { Patches::new() }

// error[E0308]: mismatched types: expected `Sig<Notes, i32>`, found `Sig<Notes, String>`
let save = button::<Notes>().on_click(SAVE.call().status(Notes::problem()));
# let _ = actions;
