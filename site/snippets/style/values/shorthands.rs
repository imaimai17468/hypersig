use hypersig::style::Style;
use hypersig::style::value::{BorderStyle, Color, Margin, MarginInline, px, raw, rem};

// One value each: `margin: auto`, `padding: 1rem`.
let card = Style::new()
    .margin(Margin::Auto)
    .padding(rem(1))
    .border_width(px(1))
    .border_style(BorderStyle::Solid)
    .border_color(Color::SILVER);

// `margin: 0 auto` has two values, so it goes through `raw`.
let centered = Style::new().margin(raw("0 auto"));

// The same box from two typed longhands: `margin-block: 0`, `margin-inline: auto`.
let typed = Style::new().margin_block(px(0)).margin_inline(MarginInline::Auto);
# assert!(!card.is_empty() && !centered.is_empty() && !typed.is_empty());
