use hypersig::{Action, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    next: i32,
    failed: Vec<i32>,
}

hypersig::routes! {
    fn actions(Router<()>);
    action SAVE: Action<Notes, i32> = post "/save" => save;
}
# async fn save((): (), _: i32) -> hypersig::Patches<Notes> { hypersig::Patches::new() }

let next = Notes::next();
// `sent` is the number `next` held when the statement ran. `on_error` runs
// after the answer arrives, when `next` has moved on, and still reads it.
let save = next.let_in_stmt(|sent| {
    SAVE.call_with(&sent)
        .on_error(|_| Notes::failed().push(sent))
        .then(next.set(next + 1))
});
# let _ = save;
