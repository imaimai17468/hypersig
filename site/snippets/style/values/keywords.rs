use hypersig::html::div;
use hypersig::style::value::{Color, CssWideKeyword};
use hypersig::style::{Property, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Note {}

let style = Style::new()
    .color(Color::CRIMSON)
    // Replaces `color: crimson`, as a second `color` call would.
    .keyword(Property::Color, CssWideKeyword::Inherit)
    .keyword(Property::Margin, CssWideKeyword::Revert);
let html = Page::new("Note", Note {}).child(div().css(style)).render();
assert!(html.contains("{color:inherit}") && !html.contains("{color:crimson}"));
assert!(html.contains("{margin:revert}"));
