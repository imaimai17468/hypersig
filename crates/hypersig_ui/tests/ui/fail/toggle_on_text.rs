use hypersig::Signals;
use hypersig_ui::toggle;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Editor {
    title: String,
}

// A toggle is pressed or not, so it binds a `bool` signal and nothing else.
fn main() {
    let _ = toggle(Editor::title());
}
