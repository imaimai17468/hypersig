use hypersig::{NodeRef, Signals};
use hypersig_ui::{ButtonVariant, Tooltip};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Editor {}

hypersig::node_ref!(const BOLD_TIP: NodeRef<Editor>);
const BOLD: Tooltip<Editor> = Tooltip::new(BOLD_TIP);
# fn main() {

// An icon button without a name: the tooltip only describes it.
let button = BOLD.icon_trigger(ButtonVariant::Outline).text("B");
# }
