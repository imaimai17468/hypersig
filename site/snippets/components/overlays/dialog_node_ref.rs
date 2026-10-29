use hypersig::{NodeRef, Signals};
use hypersig_ui::Dialog;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Project {}

// A node ref may name any element, so it cannot build a dialog.
hypersig::node_ref!(const RENAME_PANEL: NodeRef<Project>);
const RENAME: Dialog<Project> = Dialog::new(RENAME_PANEL);
# fn main() {}
