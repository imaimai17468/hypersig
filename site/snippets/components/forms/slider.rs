use hypersig::html::div;
use hypersig::{Page, Signals};
use hypersig_ui::Slider;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Player {
    volume: i32,
    muted: bool,
}

let volume = Player::volume();
let page = Page::new("Player", Player { volume: 40, muted: false }).child(
    div().child(
        Slider::new("Volume", volume)
            .range(0, 100)
            .step(5)
            // Shown beside the label and read out as "40 %".
            .value_text(volume.to_text().concat(" %"))
            .disabled(Player::muted()),
    ),
);
# let html = page.render();
# assert!(html.contains(r#"type="range" min="0" max="100" step="5" aria-valuetext="40 %""#));
