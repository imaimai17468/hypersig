use hypersig::Signals;
use hypersig_ui::Chart;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Month {
    number: i32,
    visits: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Traffic {
    months: Vec<Month>,
}

fn main() {
    // A category labels the axis, so it is a text field.
    let _ = Chart::bar("Visits", Traffic::months(), Month::number());
}
