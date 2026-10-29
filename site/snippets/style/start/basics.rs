use hypersig::html::button;
use hypersig::style::Style;
use hypersig::style::value::{Color, Cursor, FontWeight, px, rem};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {
    count: i32,
}

let buy = button::<Shop>()
    .css(
        Style::new()
            .padding_block(rem(0.5))
            .padding_inline(rem(1))
            .border_radius(px(6))
            .background_color(Color::hex(0x0018_181b))
            .color(Color::WHITE)
            .font_weight(FontWeight::Number(500.0))
            .cursor(Cursor::Pointer),
    )
    .text("Buy");

let html = Page::new("Shop", Shop { count: 0 }).child(buy).render();
// <head> holds one rule per declaration, such as `cursor: pointer`.
assert!(html.contains("{cursor:pointer}"));
assert!(html.contains("{padding-block:0.5rem}"));
