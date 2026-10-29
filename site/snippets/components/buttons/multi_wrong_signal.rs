use hypersig::Signals;
use hypersig_ui::MultiToggleGroup;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Editor {
    bold: bool,
    size: i32,
}

// A number signal, which has no pressed and not pressed.
let style = MultiToggleGroup::new("Text style")
    .item(Editor::bold(), "Bold")
    .item(Editor::size(), "Size");
# let _ = style;
