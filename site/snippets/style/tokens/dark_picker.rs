use hypersig::html::{button, div};
use hypersig::style::value::{Color, raw};
use hypersig::style::{Element, Global, Media, Style, Theme, Token};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Scheme {
    System,
    Light,
    Dark,
}

#[derive(Signals, Serialize, Deserialize)]
struct App {
    scheme: Scheme,
}

let background = Token::new("background", Color::WHITE);
let foreground = Token::new("foreground", Color::hex(0x0009_090b));
let light = Style::new()
    .token(&background, Color::WHITE)
    .token(&foreground, Color::hex(0x0009_090b))
    .color_scheme(raw("light"));
let dark = Style::new()
    .token(&background, Color::hex(0x0009_090b))
    .token(&foreground, Color::hex(0x00fa_fafa))
    .color_scheme(raw("dark"));

let scheme = App::scheme();
let html = Page::new("App", App { scheme: Scheme::System })
    // `Scheme::System`, which has no case below, follows the system.
    .theme(Theme::new().on(Media::Dark, Theme::new()
        .set(&background, Color::hex(0x0009_090b))
        .set(&foreground, Color::hex(0x00fa_fafa))))
    .global(Global::new().rule(
        Element::Body,
        Style::new().background_color(background.var()).color(foreground.var()),
    ))
    // On `<body>`, so the tokens reach its own background.
    .css_match(scheme, [(Scheme::Light, light), (Scheme::Dark, dark)])
    // The next page load starts from the stored choice.
    .persist(scheme)
    .child(
        div()
            .child(button().on_click(scheme.set(&Scheme::System)).text("System"))
            .child(button().on_click(scheme.set(&Scheme::Light)).text("Light"))
            .child(button().on_click(scheme.set(&Scheme::Dark)).text("Dark")),
    )
    .render();
# assert!(html.contains("{color-scheme:dark}"));
