use hypersig::html::p;
use hypersig::{Page, Signals};
use hypersig_ui::Collapsible;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Order {
    details: bool,
}

let details = Collapsible::new(Order::details());

// Panics: the two regions carry one id.
let html = Page::new("Order", Order { details: false })
    .child(details.content().child(p().text("Shipped on Monday.")))
    .child(details.content().child(p().text("Paid by card.")))
    .render();
