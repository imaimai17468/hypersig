use hypersig::Signals;
use hypersig_ui::Tooltip;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Editor {}

// A name written by hand could match another tooltip's, so a tooltip takes a
// `NodeRef` that `node_ref!` names after its constant.
const BOLD: Tooltip<Editor> = Tooltip::new("bold-tip");

fn main() {
    let _ = BOLD.tooltip("Ctrl+B");
}
