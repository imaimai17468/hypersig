//! A badge styled by the value of a signal, for the page on styling.

use hypersig::html::{div, span};
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::{Color, FontWeight, px, rem};
use hypersig::style::{Media, Style, Theme, Token};
use hypersig::{Page, Signals, if_else};
use hypersig_ui::{ButtonSize, ButtonVariant, button};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Status {
    Idle,
    Busy,
    Done,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Build {
    status: Status,
}

pub fn page() -> Page<Build> {
    let status = Build::status();
    let done = Token::new("done", Color::hex(0x0015_803d));
    let badge = span()
        .css(
            Style::new()
                .padding_block(rem(0.125))
                .padding_inline(rem(0.625))
                .border_radius(px(999))
                .font_size(rem(0.75))
                .font_weight(FontWeight::Number(600.0))
                .color(Color::WHITE),
        )
        .css_match(
            status,
            [
                (
                    Status::Idle,
                    Style::new().background_color(Color::hex(0x0052_525b)),
                ),
                (
                    Status::Busy,
                    Style::new().background_color(Color::hex(0x00b4_5309)),
                ),
                (Status::Done, Style::new().background_color(done.var())),
            ],
        )
        .bind_text(if_else(
            status.equals(&Status::Idle),
            "Idle",
            if_else(status.equals(&Status::Busy), "Building", "Done"),
        ));
    let set = |value: Status, label: &str| {
        button(ButtonVariant::Outline, ButtonSize::Sm)
            .on_click(status.set(&value))
            .text(label)
    };
    Page::new(
        "Build",
        Build {
            status: Status::Idle,
        },
    )
    .theme(Theme::new().on(
        Media::Dark,
        Theme::new().set(&done, Color::hex(0x0016_a34a)),
    ))
    .child(
        div().css(stack(rem(1))).child(badge).child(
            div()
                .css(hstack(rem(0.5)))
                .child(set(Status::Idle, "Idle"))
                .child(set(Status::Busy, "Busy"))
                .child(set(Status::Done, "Done")),
        ),
    )
}
