use hypersig::html::{li, ul};
use hypersig::style::Style;
use hypersig::style::value::{BorderBottomStyle, Color, px, rem};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {}

fn row() -> Style {
    Style::new()
        .padding_block(rem(0.5))
        .border_bottom_width(px(1))
        .border_bottom_style(BorderBottomStyle::Solid)
        .border_bottom_color(Color::GAINSBORO)
}

let html = Page::new("Inbox", Inbox {})
    .child(ul().children(["Rent", "Tea", "Train"].map(|text| li().css(row()).text(text))))
    .render();
// Three rows use the style, and <head> holds each declaration once.
assert_eq!(html.matches("{padding-block:0.5rem}").count(), 1);
