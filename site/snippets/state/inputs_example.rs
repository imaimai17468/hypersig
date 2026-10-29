use hypersig::html::{attr, form, input, label, p, select, span};
use hypersig::{NumberFormat, Page, Signals, if_else};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
enum Size {
    Small,
    Large,
}

#[derive(Serialize, Deserialize, PartialEq)]
enum Milk {
    None,
    Oat,
    Whole,
}

#[derive(Signals, Serialize, Deserialize)]
struct Coffee {
    size: Size,
    milk: Milk,
    shots: i32,
    to_go: bool,
    name: Option<String>,
}

const PRICE: NumberFormat = NumberFormat::new().digits(2).prefix("$");

let size = Coffee::size();
let shots = Coffee::shots();
let base = if_else(size.equals(&Size::Large), 4.0, 3.0);
let price = base + shots.to_f64() * 0.75;

let order = Page::new(
    "Coffee",
    Coffee { size: Size::Small, milk: Milk::None, shots: 1, to_go: false, name: None },
)
.child(
    form()
        .child(label().child(input().bind_radio(size, &Size::Small)).text("Small"))
        .child(label().child(input().bind_radio(size, &Size::Large)).text("Large"))
        .child(select().bind_select(
            Coffee::milk(),
            [(Milk::None, "No milk"), (Milk::Oat, "Oat milk"), (Milk::Whole, "Whole milk")],
        ))
        .child(
            label()
                .text("Shots")
                .child(input().attr(attr::TYPE, "number").attr(attr::MIN, "1").bind(shots)),
        )
        .child(
            label()
                .child(input().attr(attr::TYPE, "checkbox").bind(Coffee::to_go()))
                .text("To go"),
        )
        .child(label().text("Name for the cup").child(input().bind(Coffee::name())))
        .child(p().text("Total: ").child(span().bind_text(price.format(&PRICE)))),
);
# assert!(order.render().contains(">$3.75</span>"));
