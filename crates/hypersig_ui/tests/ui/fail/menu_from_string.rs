use hypersig::Signals;
use hypersig_ui::Menu;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Doc {}

// A name written by hand could match another menu's, so a menu takes a
// `NodeRef` that `node_ref!` names after its constant.
const ACTIONS: Menu<Doc> = Menu::new("doc-actions");

fn main() {
    let _ = ACTIONS.menu([]);
}
