//! Horizontal and vertical separators, for the separator page.

use hypersig::html::{Orientation, div, p, span};
use hypersig::style::Style;
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::{FontWeight, rem};
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, separator, token};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Profile {
    section: String,
}

pub fn page(site: Site) -> Page<Profile> {
    let section = Profile::section();
    let link = |name: &str| {
        button(ButtonVariant::Link, ButtonSize::Sm)
            .on_click(section.set(name))
            .text(name)
    };
    let initial = Profile {
        section: site
            .t(L {
                en: "Blog",
                ja: "ブログ",
            })
            .into(),
    };
    Page::new(
        site.t(L {
            en: "Profile",
            ja: "プロフィール",
        }),
        initial,
    )
    .child(
        div()
            .css(stack(rem(1)).max_width(rem(20)))
            .child(
                div()
                    .child(
                        p().css(Style::new().font_weight(FontWeight::Number(500.0)))
                            .text("hypersig"),
                    )
                    .child(
                        p().css(Style::new().color(token::muted_foreground().var()))
                            .text(site.t(L {
                                en: "Typed hypermedia for Rust.",
                                ja: "Rust のための型付きハイパーメディアです。",
                            })),
                    ),
            )
            .child(separator(Orientation::Horizontal))
            .child(
                div()
                    .css(hstack(rem(1)).height(rem(1.25)))
                    .child(link(site.t(L {
                        en: "Blog",
                        ja: "ブログ",
                    })))
                    .child(separator(Orientation::Vertical))
                    .child(link(site.t(L {
                        en: "Docs",
                        ja: "ドキュメント",
                    })))
                    .child(separator(Orientation::Vertical))
                    .child(link(site.t(L {
                        en: "Source",
                        ja: "ソースコード",
                    }))),
            )
            .child(
                p().text(site.t(L {
                    en: "Showing ",
                    ja: "表示中: ",
                }))
                .child(span().bind_text(section)),
            ),
    )
}
