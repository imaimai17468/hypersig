use hypersig::{Segment, Signals};
use hypersig_ui::Listbox;
use serde::{Deserialize, Serialize};

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Font {
    Sans,
    Serif,
    Mono,
}

#[derive(Signals, Serialize, Deserialize)]
struct Editor {
    font: Font,
}

// A selected font without an option would show nothing selected, so the
// `match` names an option for every variant.
fn main() {
    let _ = Listbox::new("Font", Editor::font(), |font| match font {
        Font::Sans => "Sans serif",
        Font::Serif => "Serif",
    });
}
