use hypersig::html::{a, p};
use hypersig::style::value::{Color, FontFamily, LineHeight, TextDecorationLine};
use hypersig::style::{Element, Global, Media, PseudoClass, Selector, Style};
use hypersig::{Page, SafeUrl, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Blog {}

let global = Global::preflight()
    .rule(Selector::Root, Style::new().on(Media::Dark, Style::new().color(Color::WHITE)))
    .rule(
        Element::Body,
        Style::new().font_family(FontFamily::SystemUi).line_height(LineHeight::Number(1.6)),
    )
    .rule(
        Element::A,
        Style::new()
            .color(Color::hex(0x0025_63eb))
            .on(PseudoClass::Hover, Style::new().text_decoration_line(TextDecorationLine::Underline)),
    )
    .rule(Element::Code, Style::new().font_family(FontFamily::Monospace));

let html = Page::new("Blog", Blog {})
    .global(global)
    .child(p().text("Read "))
    // The element's own class wins over the rule for `a`.
    .child(a().href(SafeUrl::new("/about")).css(Style::new().color(Color::GREEN)).text("about us"))
    .render();
# assert!(html.contains("@media (prefers-color-scheme: dark){:root{color:white}}"));
# assert!(html.contains("body{font-family:system-ui;line-height:1.6}"));
# assert!(html.contains("a:hover{text-decoration-line:underline}"));
