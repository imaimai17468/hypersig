use hypersig::html::div;
use hypersig::style::value::{Color, px, rem};
use hypersig::style::{Media, Style, Theme, Token};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    hue: String,
}

let surface = Token::new("surface", Color::hex(0x00ff_ffff));
let gap = Token::new("gap", rem(1));

let html = Page::new("Board", Board { hue: "#2563eb".into() })
    // For the whole page, and under a media condition.
    .theme(
        Theme::new()
            .set(&gap, rem(1.5))
            .on(Media::Dark, Theme::new().set(&surface, Color::hex(0x0018_181b))),
    )
    .child(
        div()
            // Reading a token.
            .css(Style::new().background_color(surface.var()).padding(gap.var()))
            // For one element and those inside it.
            .css(Style::new().token(&gap, px(8)))
            // From a signal, unchecked like `bind_style`.
            .bind_token(&surface, Board::hue()),
    )
    .render();
assert!(html.contains("var(--surface, #ffffff)"));
