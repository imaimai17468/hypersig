use hypersig::{NumberFormat, Page, Signals};
use hypersig_ui::{Chart, token};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Quarter {
    name: String,
    income: f64,
    costs: f64,
}

#[derive(Signals, Serialize, Deserialize)]
struct Results {
    quarters: Vec<Quarter>,
}

const THOUSANDS: NumberFormat = NumberFormat::new().prefix("$").suffix("k");

// Costs are negative, so their part of each bar reaches down from 0.
let chart = Chart::stacked_bar("Income and costs", Results::quarters(), Quarter::name())
    .series("Income", Quarter::income(), token::chart_1())
    .series("Costs", Quarter::costs(), token::chart_5())
    .format(THOUSANDS);

let quarters = vec![
    Quarter { name: "Q1".into(), income: 120.0, costs: -45.0 },
    Quarter { name: "Q2".into(), income: 150.0, costs: -60.0 },
    Quarter { name: "Q3".into(), income: 90.0, costs: -70.0 },
];
let html = Page::new("Results", Results { quarters }).child(chart).render();
# assert!(html.contains(r#"<th scope="row">Q2</th><td>$150k</td><td>-$60k</td>"#));
# assert!(html.contains(">-$100k</span>"));
