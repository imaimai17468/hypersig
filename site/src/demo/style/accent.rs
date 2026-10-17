//! A token set from a signal, for the page on tokens and themes.

use hypersig::html::div;
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::{Color, px, rem};
use hypersig::style::{Style, Token};
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, radio_group, token};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Palette {
    accent: String,
}

pub fn page() -> Page<Palette> {
    let accent = Token::new("accent-demo", Color::hex(0x0025_63eb));
    let swatch = Style::new()
        .width(rem(3))
        .height(rem(3))
        .border_radius(px(8))
        .background_color(accent.var());
    Page::new("Palette", Palette { accent: "#2563eb".into() }).child(
        div()
            .css(stack(rem(1)))
            .bind_token(&accent, Palette::accent())
            .child(radio_group(
                "Accent",
                Palette::accent(),
                [
                    ("#2563eb".to_owned(), "Blue"),
                    ("#16a34a".to_owned(), "Green"),
                    ("#e11d48".to_owned(), "Rose"),
                ],
            ))
            .child(
                div()
                    .css(wrap(rem(0.75)))
                    .child(div().css(swatch.clone()))
                    .child(div().css(swatch.border_radius(px(999))))
                    .child(
                        button(ButtonVariant::Default, ButtonSize::Default)
                            // The button reads `primary`, which takes the accent here.
                            .css(Style::new().token(&token::primary(), accent.var()))
                            .text("Accent button"),
                    ),
            ),
    )
}
