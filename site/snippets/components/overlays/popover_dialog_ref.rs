use hypersig::{DialogRef, Signals};
use hypersig_ui::Popover;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Layer {}

// A dialog ref names a `<dialog>`, and the panel is a `<div popover>`.
hypersig::dialog_ref!(const SIZE_DIALOG: DialogRef<Layer>);
const SIZE: Popover<Layer> = Popover::new(SIZE_DIALOG);
# fn main() {}
