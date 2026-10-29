use hypersig::Signals;
use hypersig_ui::ToggleGroup;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Align {
    Left,
    Right,
}

#[derive(Signals, Serialize, Deserialize)]
struct Text {
    align: Align,
}

// A value written as text, where the signal holds an `Align`.
let align = ToggleGroup::new("Alignment", Text::align())
    .item(Align::Left, "Left")
    .item("right", "Right");
# let _ = align;
