use std::time::Duration;

use hypersig::html::{div, input, p};
use hypersig::{Expr, NodeRef, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Editor {
    text: String,
    saved_at: f64,
    edits: i32,
    dirty: bool,
}

const FIELD: NodeRef<Editor> = NodeRef::new("editor-text");

let editor = div::<Editor>()
    // Focus the field when the page loads.
    .on_load(FIELD.focus())
    // Count every change to `text`, however it was made.
    .on_signal_change(
        Editor::text(),
        Editor::edits().set(Editor::edits() + 1).then(Editor::dirty().set(true)),
    )
    // Every 30 seconds, note the time of the save.
    .on_interval(
        Duration::from_secs(30),
        Editor::saved_at().set(Expr::now()).then(Editor::dirty().set(false)),
    )
    .child(input().node_ref(FIELD).bind(Editor::text()))
    .child(p().show(Editor::dirty()).text("Unsaved changes"));
# let _ = editor;
