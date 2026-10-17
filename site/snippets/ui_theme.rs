use hypersig::style::value::{Color, rem};
use hypersig::style::{Media, Style, Theme};
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, token};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {
    saved: bool,
}

// A blue primary color, lighter in the dark theme, and rounder corners.
let brand = Theme::new()
    .set(&token::primary(), Color::hex(0x0025_63eb))
    .set(&token::radius(), rem(1))
    .on(Media::Dark, Theme::new().set(&token::primary(), Color::hex(0x0060_a5fa)));

let html = Page::new("App", App { saved: false })
    .global(hypersig_ui::global())
    .theme(hypersig_ui::theme())
    .theme(brand)
    .child(
        button::<App>(ButtonVariant::Default, ButtonSize::Default)
            // One element can set a token for itself and what is inside it.
            .css(Style::new().token(&token::primary(), Color::hex(0x0016_a34a)))
            .text("Save"),
    )
    .render();
assert!(html.contains("--primary:#2563eb"));
