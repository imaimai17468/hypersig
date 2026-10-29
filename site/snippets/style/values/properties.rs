use hypersig::style::Style;
use hypersig::style::value::{
    AlignItems, Color, Display, FontFamily, FontWeight, LineHeight, deg, ms, pct, px, quoted, rem,
    url,
};

let style = Style::new()
    // Keywords are enum variants of the property's own type.
    .display(Display::Flex)
    .align_items(AlignItems::Center)
    .font_family(FontFamily::SystemUi)
    .font_weight(FontWeight::Number(600.0))
    .line_height(LineHeight::Number(1.5))
    // Lengths, percentages, times, and angles have constructors.
    .width(pct(50))
    .padding(rem(1))
    .border_width(px(1))
    .transition_duration(ms(150))
    .rotate(deg(3))
    // Colors are named, hex, RGB, or mixed.
    .color(Color::CRIMSON)
    .background_color(Color::hex(0x00fa_fafa).mix(&Color::WHITE, 50))
    .border_color(Color::rgb(228, 228, 231))
    .background_image(url("/paper.png"))
    .content(quoted("→"));
# assert!(!style.is_empty());
