use hypersig::html::{p, span};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Invoice {
    hours: f64,
    rate: f64,
    #[signals(computed)]
    net: f64,
    #[signals(computed)]
    tax: f64,
    #[signals(computed)]
    gross: f64,
}

let net = Invoice::net();
let tax = Invoice::tax();

// Each one reads only the ones declared above it.
let page = Page::new("Invoice", Invoice { hours: 10.0, rate: 80.0, net: 0.0, tax: 0.0, gross: 0.0 })
    .computed(net, Invoice::hours() * Invoice::rate())
    .computed(tax, net * 0.1)
    .computed(Invoice::gross(), net + tax)
    .child(p().text("Total: ").child(span().bind_text(Invoice::gross().to_fixed(2))));
assert_eq!(page.value_of(Invoice::gross()), 880.0);
