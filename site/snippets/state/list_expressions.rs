use hypersig::html::{div, p};
use hypersig::{Expr, NumberFormat, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Line {
    name: String,
    price: f64,
    quantity: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Order {
    lines: Vec<Line>,
}

const DOLLARS: NumberFormat = NumberFormat::new().digits(2).prefix("$");

let lines = Order::lines();
let total = lines.sum_of(|line| line.at(Line::price()) * line.at(Line::quantity()).to_f64());
let priciest: Expr<Order, Option<f64>> = lines.max_of(|line| line.at(Line::price()));
let names: Expr<Order, Vec<String>> = lines.map(|line| line.at(Line::name()));
let bulk = lines.count(|line| line.at(Line::quantity()).ge(10));

let summary = div::<Order>()
    .child(p().bind_text(lines.len().to_text().concat(" lines, ").concat(total.format(&DOLLARS))))
    .child(p().show(bulk.gt(0)).text("Bulk prices apply."))
    .child(p().bind_text(priciest.unwrap_or(0.0).format(&DOLLARS)))
    .child(p().show(lines.nth(0).is_some()).text("The first line is filled in."));
# let _ = (summary, names);
