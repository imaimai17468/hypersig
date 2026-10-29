use hypersig::html::{El, span};
use hypersig::style::Style;
use hypersig::style::value::{BorderStyle, Color, FontWeight, px, rem};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Orders {}

/// A badge tinted by `hue`: a pale fill, a darker border, and text in `hue`.
fn badge(hue: &Color, text: &str) -> El<Orders> {
    span()
        .css(
            Style::new()
                .padding_inline(rem(0.5))
                .border_width(px(1))
                .border_style(BorderStyle::Solid)
                .border_color(hue.mix(&Color::BLACK, 80))
                .border_radius(rem(1))
                .background_color(hue.opacity(12))
                .color(hue.clone())
                .font_weight(FontWeight::Bold),
        )
        .text(text)
}

let shipped = Color::hex(0x0016_a34a);
let late = Color::hex(0x00dc_2626);
let html = Page::new("Orders", Orders {})
    .child(badge(&shipped, "Shipped"))
    .child(badge(&late, "Late"))
    .render();
# assert!(html.contains("{background-color:color-mix(in srgb, #16a34a 12%, transparent)}"));
