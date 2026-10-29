use hypersig::{NodeRef, Signals};
use hypersig_ui::{Tooltip, kbd};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Editor {}

hypersig::node_ref!(const BOLD_TIP: NodeRef<Editor>);
const BOLD: Tooltip<Editor> = Tooltip::new(BOLD_TIP);
# fn main() {

// A tooltip holds text, not elements.
let tip = BOLD.tooltip(kbd().text("Ctrl+B"));
# }
