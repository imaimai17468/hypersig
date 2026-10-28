use hypersig::html::button;
use hypersig::{Page, Signals, Stmt};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Gallery {
    grid: bool,
}

// The browser takes a picture, the toggle renders, and the two cross-fade.
let switch = Stmt::view_transition(Gallery::grid().toggle());
let page = Page::new("Gallery", Gallery { grid: false })
    .child(button().on_click(switch).text("Switch layout"));
assert!(page.render().contains("transition"));
