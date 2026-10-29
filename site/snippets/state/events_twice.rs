use hypersig::html::button;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Draft {
    saved: bool,
    saves: i32,
}

let saves = Draft::saves();

let save = button::<Draft>()
    .on_click(Draft::saved().set(true))
    // A second statement for the same event runs after the first.
    .on_click(saves.set(saves + 1))
    .text("Save");
# let html = Page::new("Draft", Draft { saved: false, saves: 0 }).child(save).render();
# let _ = html;
