use hypersig::Signals;
use hypersig_ui::Dialog;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {}

// An id with a space would not match the `commandfor` of its buttons.
const EDIT: Dialog<Profile> = Dialog::new("edit profile");

fn main() {
    let _ = EDIT.dialog("Edit", "Edit your profile.", "");
}
