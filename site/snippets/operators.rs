use hypersig::html::p;
use hypersig::{Expr, Signals, if_else};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Order {
    price: f64,
    quantity: i32,
    coupon: String,
}

let price = Order::price();
let quantity = Order::quantity();
let coupon = Order::coupon();

let subtotal: Expr<Order, f64> = price * quantity.to_f64();
let discounted = coupon.trim().to_uppercase().equals("TEA10");
let total = if_else(discounted.clone(), subtotal.clone() * 0.9, subtotal);

let summary = p::<Order>()
    .bind_text(total.to_fixed(2))
    .show(quantity.gt(0).and(price.ge(0.0)));
# let _ = summary;
