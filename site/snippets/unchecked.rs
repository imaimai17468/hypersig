use hypersig::html::{Stylesheet, div};
use hypersig::style::Style;
use hypersig::style::value::raw;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Legacy {
    open: bool,
}

// A value with several parts goes through `raw`, which the browser checks.
let card = Style::new()
    .margin(raw("0 auto"))
    .box_shadow(raw("0 1px 3px rgb(0 0 0 / 0.1)"))
    .width(raw("calc(100% - 2rem)"));

// An existing stylesheet keeps working. `class` fails to compile unless the
// CSS has a selector for the class.
const LEGACY: Stylesheet = Stylesheet::new(".panel { padding: 1rem } .open { display: block }");
const PANEL: hypersig::html::Class = LEGACY.class("panel");
const OPEN: hypersig::html::Class = LEGACY.class("open");

let html = Page::new("Legacy", Legacy { open: false })
    .style(LEGACY)
    .child(div().css(card).class(PANEL).class_when(OPEN, Legacy::open()))
    .render();
assert!(html.contains("panel"));
