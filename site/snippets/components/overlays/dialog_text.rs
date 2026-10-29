use hypersig::html::p;
use hypersig::{DialogRef, Signals};
use hypersig_ui::Dialog;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Project {}

hypersig::dialog_ref!(const RENAME_DIALOG: DialogRef<Project>);
const RENAME: Dialog<Project> = Dialog::new(RENAME_DIALOG);
# fn main() {

// The close button in the corner has no name.
let dialog = RENAME.dialog("Rename project", "Pick a new name.", p().text("…"));
# }
