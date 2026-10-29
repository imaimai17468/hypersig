use hypersig::html::{El, button, div};
use hypersig::style::Style;
use hypersig::style::value::{Color, Cursor, Display, FontWeight, MarginLeft, px, rem};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Settings {}

#[derive(Clone, Copy)]
enum Tone {
    Neutral,
    Danger,
}

/// The style of a button of `tone`: the shared base, then what the tone changes.
fn button_style(tone: Tone) -> Style {
    let base = Style::new()
        .padding_block(rem(0.5))
        .padding_inline(rem(1))
        .border_width(px(0))
        .border_radius(px(6))
        .font_weight(500)
        .cursor(Cursor::Pointer);
    base.merge(match tone {
        Tone::Neutral => Style::new()
            .background_color(Color::hex(0x00f4_f4f5))
            .color(Color::hex(0x0018_181b)),
        Tone::Danger => Style::new()
            .background_color(Color::hex(0x00dc_2626))
            .color(Color::WHITE)
            .font_weight(FontWeight::Bold),
    })
}

fn actions() -> El<Settings> {
    div()
        .css(Style::new().display(Display::Flex).gap(rem(0.5)))
        .child(button().css(button_style(Tone::Neutral)).text("Cancel"))
        .child(
            button()
                .css(button_style(Tone::Danger))
                .css(Style::new().margin_left(MarginLeft::Auto))
                .text("Delete account"),
        )
}

let html = Page::new("Settings", Settings {}).child(actions()).render();
# assert!(html.contains("{font-weight:bold}") && html.contains("{font-weight:500}"));
# assert!(html.contains("{margin-left:auto}"));
