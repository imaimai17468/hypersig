use hypersig::Signals;
use hypersig_ui::DateField;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Booking {
    arrival: String,
}

// A date kept as text, such as "2026-04-01".
let arrival = DateField::new("Arrival", Booking::arrival());
# let _ = arrival;
