use hypersig::html::{Id, p};
use hypersig::{Page, Signals};
use hypersig_ui::Accordion;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Help {}

const SHIPPING: Id = Id::new("shipping");

// Any number of these sections can be open at once.
let notes = Accordion::new()
    .item("Release notes", p().text("Version 2 adds dark mode."))
    .item("Known issues", p().text("Printing drops the header."));

// Opening one of these sections closes the other.
let shipping = Accordion::exclusive(SHIPPING)
    .item("How long does delivery take?", p().text("Three to five working days."))
    .item("Can I change the address?", p().text("Until the parcel leaves the warehouse."));

let html = Page::new("Help", Help {})
    .child(notes)
    .child(shipping)
    .render();
# assert!(html.contains(r#"<div id="shipping">"#));
# assert_eq!(html.matches(r#" name="shipping""#).count(), 2);
# assert_eq!(html.matches("<details").count(), 4);
# assert!(html.contains("<summary"));
