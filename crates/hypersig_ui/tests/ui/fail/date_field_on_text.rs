use hypersig::Signals;
use hypersig_ui::DateField;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Booking {
    arrival: String,
}

fn main() {
    // A date field holds a `CalendarDate`, so a text signal cannot be its value.
    let _ = DateField::new("Arrival", Booking::arrival()).render();
}
