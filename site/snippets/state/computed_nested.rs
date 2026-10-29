use hypersig::html::span;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Totals {
    net: f64,
    #[signals(computed)]
    gross: f64,
}

#[derive(Signals, Serialize, Deserialize)]
struct Invoice {
    totals: Totals,
}

let totals = Invoice::totals();
// `at` refuses a computed field, and `at_computed` takes one.
let gross = totals.at_computed(Totals::gross());

let page = Page::new("Invoice", Invoice { totals: Totals { net: 100.0, gross: 0.0 } })
    .computed(gross, totals.at(Totals::net()) * 1.1)
    .child(span().bind_text(gross.to_fixed(2)));
# assert!(page.render().contains(">110.00</span>"));
