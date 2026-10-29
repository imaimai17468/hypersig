use hypersig::{DialogRef, Signals};
use hypersig_ui::{Dialog, DialogText, dialog_footer};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Project {}

hypersig::dialog_ref!(const DELETE_DIALOG: DialogRef<Project>);
const DELETE: Dialog<Project> = Dialog::new(DELETE_DIALOG);
# fn main() {

// An alert dialog has no corner button to name.
let dialog = DELETE.alert_dialog(
    "Delete this project?",
    "This cannot be undone.",
    dialog_footer(),
    &DialogText::default(),
);
# }
