use hypersig::html::div;
use hypersig::{NodeRef, Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Menu, MenuItem};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Doc {
    archived: bool,
}

hypersig::node_ref!(const DOC_ACTIONS: NodeRef<Doc>);
const ACTIONS: Menu<Doc> = Menu::new(DOC_ACTIONS);
# fn main() {

let menu = || ACTIONS.menu([MenuItem::new("Archive", Doc::archived().set(true))]);
let page = Page::new("Doc", Doc { archived: false }).child(
    div()
        .child(ACTIONS.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Actions"))
        .child(menu())
        // The same menu a second time.
        .child(menu()),
);
// Panics: "two elements carry the id `...::DOC_ACTIONS` where the runtime can reach both".
let _ = page.render();
# }
