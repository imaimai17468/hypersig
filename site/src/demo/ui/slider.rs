//! A slider with a unit, for the slider page.

use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::Slider;
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Player {
    volume: i32,
}

pub fn page(site: Site) -> Page<Player> {
    let volume = Player::volume();
    Page::new(
        site.t(L {
            en: "Player",
            ja: "プレーヤー",
        }),
        Player { volume: 40 },
    )
    .child(
        div().css(Style::new().max_width(rem(20))).child(
            Slider::new(
                site.t(L {
                    en: "Volume",
                    ja: "音量",
                }),
                volume,
            )
            .step(5)
            .value_text(volume.to_text().concat(" %")),
        ),
    )
}
