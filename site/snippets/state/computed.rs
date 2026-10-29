use hypersig::html::span;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    price: f64,
    quantity: i32,
    // The value given here reaches the page only where the server cannot
    // evaluate the expression.
    #[signals(computed)]
    total: f64,
}

let html = Page::new("Cart", Cart { price: 2.5, quantity: 3, total: 0.0 })
    .computed(Cart::total(), Cart::price() * Cart::quantity().to_f64())
    .child(span().bind_text(Cart::total().to_text()))
    .render();
assert!(html.contains(">7.5</span>"));
