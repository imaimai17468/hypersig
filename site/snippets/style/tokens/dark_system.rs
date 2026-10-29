use hypersig::html::p;
use hypersig::style::value::{Color, ColorScheme};
use hypersig::style::{Element, Global, Media, Selector, Style, Theme, Token};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Blog {}

// The defaults are the light values.
let background = Token::new("background", Color::WHITE);
let foreground = Token::new("foreground", Color::hex(0x0009_090b));

let html = Page::new("Blog", Blog {})
    .theme(Theme::new().on(
        Media::Dark,
        Theme::new()
            .set(&background, Color::hex(0x0009_090b))
            .set(&foreground, Color::hex(0x00fa_fafa)),
    ))
    .global(
        Global::new()
            // Scrollbars and form controls follow the system too.
            .rule(Selector::Root, Style::new().color_scheme(ColorScheme::LightDark))
            .rule(
                Element::Body,
                Style::new().background_color(background.var()).color(foreground.var()),
            ),
    )
    .child(p().text("Hello"))
    .render();
# assert!(html.contains("@media (prefers-color-scheme: dark){:root{--background:#09090b;--foreground:#fafafa}}"));
# assert!(html.contains(":root{color-scheme:light dark}"));
