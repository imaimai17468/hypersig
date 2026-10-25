use hypersig::Signals;
use hypersig_ui::{ButtonSize, ButtonVariant, Dialog};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {}

// A name written by hand could match another dialog's, so a dialog takes a
// `DialogRef` that `dialog_ref!` names after its constant.
const EDIT: Dialog<Profile> = Dialog::new("edit-profile");

fn main() {
    let _ = EDIT.trigger(ButtonVariant::Outline, ButtonSize::Default);
}
