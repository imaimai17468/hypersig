use hypersig::Signals;
use hypersig_ui::switch;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Settings {
    // "on" or "off"
    airplane: String,
}

// A text signal, where a switch needs on or off.
let airplane = switch("Airplane mode", Settings::airplane());
# let _ = airplane;
