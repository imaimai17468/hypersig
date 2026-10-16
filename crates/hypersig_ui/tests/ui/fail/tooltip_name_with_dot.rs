use hypersig::Signals;
use hypersig_ui::Tooltip;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Editor {}

const BOLD: Tooltip<Editor> = Tooltip::new("bold.tip");

fn main() {
    let _ = BOLD.tooltip("Ctrl+B");
}
