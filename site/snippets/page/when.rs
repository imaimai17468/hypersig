use hypersig::html::{attr, button, div, input, p, when};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    open: bool,
}

let open = Search::open();
let page = Page::new("Search", Search { open: false }).child(
    div()
        .child(button().on_click(open.toggle()).text("Search"))
        // Hidden while closed, and still in the document.
        .child(p().show(open).text("Type a name or an email."))
        // Out of the document while closed, so the field starts empty each time.
        .child(when(open, input().attr(attr::TYPE, "search"))),
);
# let _ = page.render();
