use hypersig::html::{Event, div, input, p};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
    editing: bool,
}

let editing = Address::editing();

// Tab from the street to the city keeps focus inside the `div`,
// so neither statement runs.
let fields = div::<Address>()
    .on(Event::Focus, editing.set(true))
    .on_blur(editing.set(false))
    .child(input().bind(Address::street()))
    .child(input().bind(Address::city()))
    .child(p().show(editing).text("Editing the address"));
# let html = Page::new("Address", Address { street: String::new(), city: String::new(), editing: false }).child(fields).render();
# let _ = html;
