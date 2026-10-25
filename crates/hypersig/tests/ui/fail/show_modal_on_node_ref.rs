use hypersig::html::button;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Palette {
    query: String,
}

hypersig::node_ref!(const PALETTE: NodeRef<Palette>);

fn main() {
    // A `NodeRef` may sit on any element, so only a `DialogRef` opens a dialog.
    let _ = button::<Palette>().on_click(PALETTE.show_modal());
}
