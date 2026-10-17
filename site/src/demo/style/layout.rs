//! Layout patterns, for the page on layout patterns.

use hypersig::html::{div, span};
use hypersig::style::pattern::{center, circle, grid, hstack, spacer, stack};
use hypersig::style::value::{Color, FontWeight, px, rem};
use hypersig::style::Style;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, badge, button, token, BadgeVariant};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Team {
    columns: i32,
}

pub fn page() -> Page<Team> {
    let tile = || {
        div().css(
            center()
                .height(rem(3))
                .border_radius(px(6))
                .background_color(token::muted().var()),
        )
    };
    Page::new("Team", Team { columns: 0 }).child(
        div()
            .css(stack(rem(1.25)))
            .child(
                div()
                    .css(hstack(rem(0.75)))
                    .child(
                        div()
                            .css(
                                circle(rem(2.5))
                                    .background_color(Color::hex(0x0018_181b))
                                    .color(Color::WHITE),
                            )
                            .text("AL"),
                    )
                    .child(
                        span()
                            .css(Style::new().font_weight(FontWeight::Number(600.0)))
                            .text("Ada Lovelace"),
                    )
                    .child(badge(BadgeVariant::Secondary).text("Admin"))
                    .child(div().css(spacer()))
                    .child(button(ButtonVariant::Outline, ButtonSize::Sm).text("Message")),
            )
            .child(
                div()
                    .css(grid(3, rem(0.75)))
                    .children((1..=6).map(|n| tile().text(n.to_string()))),
            ),
    )
}
