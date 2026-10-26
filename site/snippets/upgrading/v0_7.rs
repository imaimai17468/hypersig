# use hypersig::html::{button, span};
# use hypersig::{Page, Signals};
# use serde::{Deserialize, Serialize};
#
#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    price: f64,
    quantity: i32,
    #[signals(computed)] //+
    total: f64,
}

let page = Page::new("Cart", Cart { price: 2.5, quantity: 3, total: 0.0 })
    .computed(Cart::total(), Cart::price() * Cart::quantity().to_f64())
    .child(span().bind_text(Cart::total().to_text()))
//-    .child(button().on_click(Cart::total().set(0.0)).text("Clear"));
    .child(button().on_click(Cart::quantity().set(0)).text("Clear")); //+
# assert!(page.render().contains(">7.5</span>"));
