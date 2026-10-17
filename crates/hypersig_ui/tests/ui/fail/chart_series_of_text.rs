use hypersig::Signals;
use hypersig_ui::{Chart, ChartScale, token};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Month {
    name: String,
    note: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Traffic {
    months: Vec<Month>,
    scale: ChartScale,
}

fn main() {
    // A series draws a number, so a text field cannot be one.
    let _ = Chart::bar("Visits", Traffic::months(), Traffic::scale(), Month::name())
        .series("Note", Month::note(), token::chart_1());
}
