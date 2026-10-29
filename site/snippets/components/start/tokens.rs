use hypersig::html::div;
use hypersig::style::value::{Color, px, rem};
use hypersig::style::{Media, Style, Theme};
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, token};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {}

// A blue primary color, lighter in the dark theme, and rounder corners.
let brand = Theme::new()
    .set(&token::primary(), Color::hex(0x0025_63eb))
    .set(&token::radius(), rem(1))
    .on(Media::Dark, Theme::new().set(&token::primary(), Color::hex(0x0060_a5fa)));

// An element of your own that reads the same tokens as the components.
let note = div::<App>()
    .css(
        Style::new()
            .padding(rem(1))
            .border_width(px(1))
            .border_color(token::border().var())
            .border_radius(token::radius().var())
            .background_color(token::muted().var())
            .color(token::muted_foreground().var()),
    )
    .text("Drafts are kept for 30 days.");

let html = Page::new("App", App {})
    .global(hypersig_ui::global())
    .theme(hypersig_ui::theme())
    // Added after `theme()`, so its dark `primary` replaces the one from `theme()`.
    .theme(brand)
    .child(note)
    .child(
        button::<App>(ButtonVariant::Default, ButtonSize::Default)
            // A green primary for this button and what is inside it.
            .css(Style::new().token(&token::primary(), Color::hex(0x0016_a34a)))
            .text("Save"),
    )
    .render();
# assert!(html.contains("--primary:#2563eb"));
# assert!(html.contains("--primary:#60a5fa"));
# assert!(!html.contains("--primary:#e4e4e7"));
# assert!(html.contains("var(--muted, #f4f4f5)"));
