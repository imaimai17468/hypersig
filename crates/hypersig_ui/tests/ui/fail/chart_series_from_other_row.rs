use hypersig::Signals;
use hypersig_ui::{Chart, token};
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
}

fn main() {
    let _ = Chart::bar("Visits", Traffic::months(), Month::name())
        .series("Sales", Store::sales(), token::chart_1());
}
