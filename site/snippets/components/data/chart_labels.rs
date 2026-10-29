use hypersig::Signals;
use hypersig_ui::Chart;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Day {
    date: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Growth {
    days: Vec<Day>,
}

// Panics: a chart labels at least one row.
let chart = Chart::line("Signups", Growth::days(), Day::date()).max_labels(0);
