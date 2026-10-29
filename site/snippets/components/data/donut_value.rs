use hypersig::Signals;
use hypersig_ui::DonutChart;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Plan {
    name: String,
    revenue: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Revenue {
    plans: Vec<Plan>,
}

// error[E0277]: the browser does not do arithmetic on `String`
let chart = DonutChart::new("Revenue by plan", Revenue::plans(), Plan::name(), Plan::revenue());
