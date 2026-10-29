use hypersig::Signals;
use hypersig_ui::Slider;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Player {
    volume: f64,
}

// An `f64` signal, where a slider takes an `i32`.
let volume = Slider::new("Volume", Player::volume());
# let _ = volume;
