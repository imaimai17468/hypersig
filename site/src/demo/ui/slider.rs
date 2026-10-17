//! A slider with a unit, for the slider page.

use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::Slider;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Player {
    volume: i32,
}

pub fn page() -> Page<Player> {
    let volume = Player::volume();
    Page::new("Player", Player { volume: 40 }).child(
        div().css(Style::new().max_width(rem(20))).child(
            Slider::new("Volume", volume)
                .step(5)
                .value_text(volume.to_text().concat(" %")),
        ),
    )
}
