use hypersig::style::Style;
use hypersig::style::value::{Color, Cursor, px, rem};

fn button() -> Style {
    Style::new()
        .padding_inline(rem(1))
        .border_radius(px(6))
        .background_color(Color::hex(0x0018_181b))
        .color(Color::WHITE)
        .cursor(Cursor::Pointer)
}

// The added style's `background-color` replaces the first one's.
let danger = button().merge(Style::new().background_color(Color::hex(0x00dc_2626)));
assert_eq!(danger, button().background_color(Color::hex(0x00dc_2626)));
