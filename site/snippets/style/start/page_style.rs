use hypersig::html::{Stylesheet, button};
use hypersig::style::Style;
use hypersig::style::value::Color;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {}

const PRINT: Stylesheet = Stylesheet::new("button { color: black }");

// The button is black: the rule of `Page::style` overrides the class.
let page = Page::new("Shop", Shop {})
    .style(PRINT)
    .child(button().css(Style::new().color(Color::CRIMSON)).text("Buy"));
# let html = page.render();
# assert!(html.contains("button { color: black }") && html.contains("{color:crimson}"));
