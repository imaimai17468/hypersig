use hypersig::Signals;
use hypersig_ui::Menu;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Doc {}

const ACTIONS: Menu<Doc> = Menu::new("");

fn main() {
    let _ = ACTIONS.menu([]);
}
