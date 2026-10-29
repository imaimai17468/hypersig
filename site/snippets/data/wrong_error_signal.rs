# use hypersig::html::{button, form};
# use hypersig::{Action, Patches, Signals};
# use serde::{Deserialize, Serialize};
# async fn add((): (), _draft: String) -> Patches<Notes> {
#     Patches::new()
# }
#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    draft: String,
    failures: i32,
}

hypersig::routes! {
    fn actions(Router<()>);
    action ADD: Action<Notes, String> = post "/notes" => add;
}

// `Call::error` writes text, and `failures` holds an `i32`.
let _form = form()
    .on_submit(ADD.call_with(Notes::draft()).error(Notes::failures()))
    .child(button().text("Add"));
# let _ = actions;
