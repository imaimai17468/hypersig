//! Layout patterns, for the page on layout patterns.

use hypersig::html::{div, span};
use hypersig::style::Style;
use hypersig::style::pattern::{center, circle, grid, hstack, spacer, stack};
use hypersig::style::value::{Color, FontWeight, px, rem};
use hypersig::{Page, Signals};
use hypersig_ui::{BadgeVariant, ButtonSize, ButtonVariant, badge, button, token};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Team {
    columns: i32,
}

pub fn page(site: Site) -> Page<Team> {
    let tile = || {
        div().css(
            center()
                .height(rem(3))
                .border_radius(px(6))
                .background_color(token::muted().var()),
        )
    };
    Page::new(
        site.t(L {
            en: "Team",
            ja: "チーム",
        }),
        Team { columns: 0 },
    )
    .child(
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
                    .child(badge(BadgeVariant::Secondary).text(site.t(L {
                        en: "Admin",
                        ja: "管理者",
                    })))
                    .child(div().css(spacer()))
                    .child(
                        button(ButtonVariant::Outline, ButtonSize::Sm).text(site.t(L {
                            en: "Message",
                            ja: "メッセージ",
                        })),
                    ),
            )
            .child(
                div()
                    .css(grid(3, rem(0.75)))
                    .children((1..=6).map(|n| tile().text(n.to_string()))),
            ),
    )
}
