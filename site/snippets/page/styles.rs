use hypersig::html::button;
use hypersig::style::value::{Color, Cursor, rem};
use hypersig::style::{Media, PseudoClass, Style, Theme, Token};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    busy: bool,
}

let accent = Token::new("accent", Color::hex(0xb83f45));
let page = Page::new("Cart", Cart { busy: false })
    .theme(Theme::new().on(Media::Dark, Theme::new().set(&accent, Color::hex(0xff6b6b))))
    .child(
        button()
            .css(
                Style::new()
                    .padding(rem(0.5))
                    .color(accent.var())
                    .cursor(Cursor::Pointer)
                    .on(PseudoClass::Hover, Style::new().color(Color::CRIMSON))
                    .on(Media::Md, Style::new().padding(rem(1))),
            )
            .css_when(Style::new().opacity(0.5), Cart::busy())
            .text("Buy"),
    );
# let _ = page.render();
