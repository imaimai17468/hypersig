use hypersig::html::{div, p, span};
use hypersig::{Page, Signals};
use hypersig_ui::NumberField;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Order {
    quantity: i32,
    tip: f64,
}

let quantity = Order::quantity();
let page = Page::new("Order", Order { quantity: 1, tip: 0.0 }).child(
    div()
        // `min(1)` is 0 or more, so a phone shows a keyboard of digits.
        .child(
            NumberField::new("Quantity", quantity)
                .min(1)
                .max(10)
                .description("Up to 10 per order.")
                .required(),
        )
        // An `f64` field without a step takes any decimal.
        .child(NumberField::new("Tip", Order::tip()).min(0.0))
        .child(p().text("Items: ").child(span().bind_text(quantity.to_text()))),
);
# let html = page.render();
# assert!(html.contains(r#"type="number" inputmode="numeric" min="1" max="10""#));
# assert!(html.contains(r#"inputmode="decimal" min="0" step="any""#));
