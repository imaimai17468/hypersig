use hypersig::html::{button, div, input};
use hypersig::{DialogRef, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Palette {
    query: String,
}

hypersig::dialog_ref!(const PALETTE: DialogRef<Palette>);
# fn main() {

let page = div::<Palette>()
    .child(button().on_click(PALETTE.show_modal()).text("Commands"))
    .child(
        // The `<dialog>` element that `PALETTE` names.
        PALETTE
            .dialog()
            .child(input().bind(Palette::query()))
            .child(button().on_click(PALETTE.close()).text("Close")),
    );
# let _ = page.render();
# }
