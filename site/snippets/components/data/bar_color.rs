use hypersig::Signals;
use hypersig_ui::Chart;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Quarter {
    name: String,
    income: f64,
}

#[derive(Signals, Serialize, Deserialize)]
struct Results {
    quarters: Vec<Quarter>,
}

// error[E0308]: mismatched types: expected `Token<Color>`, found `&str`
let chart = Chart::bar("Income", Results::quarters(), Quarter::name())
    .series("Income", Quarter::income(), "green");
