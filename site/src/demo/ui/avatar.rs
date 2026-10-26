//! An avatar whose picture a signal holds, and one with initials alone, for
//! the avatar page.

use hypersig::html::div;
use hypersig::style::pattern::hstack;
use hypersig::style::value::rem;
use hypersig::{Page, SafeUrl, Signals};
use hypersig_ui::{Avatar, ButtonSize, ButtonVariant, button};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Team {
    picture: SafeUrl,
}

const LOGO: SafeUrl = SafeUrl::new("https://imaimai17468.github.io/hypersig/icon.svg");
const MISSING: SafeUrl = SafeUrl::new("https://imaimai17468.github.io/hypersig/missing.png");

pub fn page(site: Site) -> Page<Team> {
    let picture = Team::picture();
    Page::new(
        site.t(L {
            en: "Team",
            ja: "チーム",
        }),
        Team { picture: LOGO },
    )
    .child(
        div()
            .css(hstack(rem(0.75)))
            .child(Avatar::new("hypersig", "HS").picture(picture))
            .child(Avatar::new("Grace Hopper", "GH"))
            .child(
                button(ButtonVariant::Outline, ButtonSize::Sm)
                    .on_click(picture.set(MISSING))
                    .text(site.t(L {
                        en: "Point at a missing picture",
                        ja: "存在しない画像を指す",
                    })),
            )
            .child(
                button(ButtonVariant::Outline, ButtonSize::Sm)
                    .on_click(picture.set(LOGO))
                    .text(site.t(L {
                        en: "Restore",
                        ja: "元に戻す",
                    })),
            ),
    )
}
