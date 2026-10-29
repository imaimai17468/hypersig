use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::raw;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Article {}

// Values with several parts, which no generated type spells out.
let card = Style::new()
    .margin(raw("0 auto"))
    .box_shadow(raw("0 1px 3px rgb(0 0 0 / 0.1)"))
    .width(raw("calc(100% - 2rem)"))
    .grid_template_columns(raw("200px minmax(0, 1fr)"));

let html = Page::new("Article", Article {}).child(div().css(card)).render();
assert!(html.contains("{margin:0 auto}"));
assert!(html.contains("{width:calc(100% - 2rem)}"));
