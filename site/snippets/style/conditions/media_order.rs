use hypersig::style::value::Color;
use hypersig::style::{Media, PseudoClass, Sheet, Style};

let link = Style::new()
    .color(Color::GRAY)
    .on(PseudoClass::Hover, Style::new().color(Color::BLUE))
    .on(
        Media::Md,
        Style::new()
            .color(Color::BLACK)
            // Without this, the link stays black while hovered from 48rem up.
            .on(PseudoClass::Hover, Style::new().color(Color::BLUE)),
    );
# let unnested = Style::new()
#     .on(PseudoClass::Hover, Style::new().color(Color::BLUE))
#     .on(Media::Md, Style::new().color(Color::BLACK));
# let mut sheet = Sheet::new();
# sheet.add(&unnested);
# let css = sheet.to_css();
# let at = |css: &str, rule: &str| css.find(rule).unwrap_or_else(|| panic!("no {rule}"));
# assert!(at(&css, ":hover{color:blue}") < at(&css, "{color:black}"));
# let mut sheet = Sheet::new();
# sheet.add(&link);
# let css = sheet.to_css();
# assert!(Some(at(&css, "{color:black}")) < css.rfind(":hover{color:blue}"));
