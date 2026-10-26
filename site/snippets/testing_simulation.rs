use hypersig::html::{button, div, input, p};
use hypersig::{Action, NodeRef, Page, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
    note: String,
    saving: bool,
}

hypersig::routes! {
    fn actions(Router<()>);
    action SAVE: Action<Counter, (i32, String)> = post "/save" => save;
}
# async fn save((): (), _: (i32, String)) -> Patches<Counter> { Patches::new() }

hypersig::node_ref!(const ADD: NodeRef<Counter>);
hypersig::node_ref!(const NOTE: NodeRef<Counter>);
hypersig::node_ref!(const SHOWN: NodeRef<Counter>);
hypersig::node_ref!(const STORE: NodeRef<Counter>);
# fn main() {
# let _ = actions;

let (count, note) = (Counter::count(), Counter::note());
let page = Page::new("Counter", Counter { count: 0, note: String::new(), saving: false })
    .child(
        div()
            .child(p().node_ref(SHOWN).bind_text(count.to_text()))
            .child(button().node_ref(ADD).on_click(count.set(count + 1)).text("Add"))
            .child(input().node_ref(NOTE).bind(note))
            .child(
                button()
                    .node_ref(STORE)
                    .on_click(SAVE.call_with((count, note)).indicator(Counter::saving()))
                    .text("Save"),
            ),
    );

let mut browser = page.simulate();
browser.click(ADD);
browser.click(ADD);
browser.input(NOTE, "twice");
assert_eq!(browser.value_of(count), 2);
assert_eq!(browser.element(SHOWN).text(), "2");

// The call is recorded with the arguments it sends, and never answered.
let effects = browser.click(STORE);
assert_eq!(effects.calls(&SAVE), [(2, "twice".to_owned())]);
assert!(browser.value_of(Counter::saving()));
# }
