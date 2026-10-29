use hypersig::Signals;
use hypersig_ui::{Chart, token};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Day {
    date: String,
    signups: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Growth {
    days: Vec<Day>,
}

// error[E0277]: the browser does not do arithmetic on `String`
let chart = Chart::line("Signups", Growth::days(), Day::date())
    .series("Date", Day::date(), token::chart_1());
