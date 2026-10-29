use hypersig::html::{El, div, span};
use hypersig::{FromPage, Page, Read, Sig, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    price: f64,
    quantity: i32,
    #[signals(computed)]
    total: f64,
}

// Takes a plain field and a computed one alike, and cannot write either.
fn amount<S, R: FromPage>(value: Sig<S, f64, R>) -> El<S> {
    let value: Sig<S, f64, Read> = value.read();
    span().bind_text(value.to_fixed(2))
}

let page = Page::new("Cart", Cart { price: 2.5, quantity: 3, total: 0.0 })
    .computed(Cart::total(), Cart::price() * Cart::quantity().to_f64())
    .child(div().child(amount(Cart::price())).child(amount(Cart::total())));
# assert!(page.render().contains(">7.50</span>"));
