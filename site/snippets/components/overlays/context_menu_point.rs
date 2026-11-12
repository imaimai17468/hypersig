use hypersig::html::li;
use hypersig::{NodeRef, Signals};
use hypersig_ui::ContextMenu;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Chat {
    picked: i32,
}

hypersig::node_ref!(const MESSAGE_ACTIONS: NodeRef<Chat>);
const MESSAGE_MENU: ContextMenu<Chat> = ContextMenu::new(MESSAGE_ACTIONS);
# fn main() {

// `open_at` takes a position, and `picked` is an `i32`.
let row = li().on_context_menu(|_| MESSAGE_MENU.open_at(Chat::picked()));
# }
