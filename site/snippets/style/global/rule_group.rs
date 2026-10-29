use hypersig::style::value::TextDecorationLine;
use hypersig::style::{Condition, Element, Global, PseudoClass, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Blog {}

let global = Global::new().rule(
    Element::A,
    Style::new().on(
        Condition::Group(PseudoClass::Hover),
        Style::new().text_decoration_line(TextDecorationLine::Underline),
    ),
);
// Panics: a global rule takes no group or peer condition
let html = Page::new("Blog", Blog {}).global(global).render();
# let _ = html;
