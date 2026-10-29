use hypersig::style::value::{Color, TextDecorationLine};
use hypersig::style::{PseudoClass, Style};

let link = Style::new()
    .color(Color::hex(0x0025_63eb))
    // `:hover`.
    .on(PseudoClass::Hover, Style::new().text_decoration_line(TextDecorationLine::Underline))
    // `:visited`, and `:visited:hover` for both at once.
    .on(
        PseudoClass::Visited,
        Style::new()
            .color(Color::hex(0x006d_28d9))
            .on(PseudoClass::Hover, Style::new().color(Color::hex(0x004c_1d95))),
    );
# let swapped = Style::new().on(
#     PseudoClass::Hover,
#     Style::new().on(PseudoClass::Visited, Style::new().color(Color::hex(0x004c_1d95))),
# );
# let names: Vec<String> = link.class_names().collect();
# assert!(names.contains(&swapped.class_names().next().unwrap_or_default()));
# let mut sheet = hypersig::style::Sheet::new();
# sheet.add(&link);
# assert!(sheet.to_css().contains(":visited:hover{color:#4c1d95}"));
