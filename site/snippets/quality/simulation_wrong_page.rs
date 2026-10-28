use hypersig::html::button;
use hypersig::{NodeRef, Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Settings {
    dark: bool,
}

hypersig::node_ref!(const ADD: NodeRef<Counter>);
hypersig::node_ref!(const THEME: NodeRef<Settings>);

# fn main() {
let page = Page::new("Counter", Counter { count: 0 })
    .child(button().node_ref(ADD).on_click(Counter::count().set(1)).text("Add"));
let mut browser = page.simulate();
// error[E0277]: the trait bound `NodeRef<Settings>: hypersig::testing::Target<Counter>`
// is not satisfied
browser.click(THEME);
# }
