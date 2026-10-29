use hypersig::html::button;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Order {
    quantity: i32,
    summary: String,
}

let quantity = Order::quantity();
let summary = Order::summary();

// The second statement reads the quantity the first one wrote.
let add = quantity
    .set(quantity + 1)
    .then(summary.set(quantity.to_text().concat(" in the cart")));
# let tested = add.clone();
let plus = button::<Order>().on_click(add).text("+1");
# use hypersig::html::Id;
# let page = hypersig::Page::new("Order", Order { quantity: 0, summary: String::new() })
#     .child(button().id(Id::new("add")).on_click(tested));
# let mut browser = page.simulate();
# let _ = browser.click(Id::new("add"));
# assert_eq!(browser.value_of(summary), "1 in the cart");
# let _ = plus;
