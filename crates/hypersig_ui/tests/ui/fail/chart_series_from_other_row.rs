use hypersig::Signals;
use hypersig_ui::{Chart, ChartScale, token};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Month {
    name: String,
    visits: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Store {
    sales: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Traffic {
    months: Vec<Month>,
    scale: ChartScale,
}

fn main() {
    let _ = Chart::bar("Visits", Traffic::months(), Traffic::scale(), Month::name())
        .series("Sales", Store::sales(), token::chart_1());
}
