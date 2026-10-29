use hypersig::html::button;
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    saving: bool,
    problem: String,
}

hypersig::routes! {
    fn pages(Router<()>);
    action SAVE: Action<Notes> = post "/save" => save;
}
# async fn save((): (), (): ()) -> Patches<Notes> { Patches::new() }

// error[E0308]: mismatched types: expected `Sig<Notes, bool>`, found `Sig<Notes, String>`
let save = button::<Notes>().on_click(SAVE.call().indicator(Notes::problem()));
# let _ = (save, pages);
