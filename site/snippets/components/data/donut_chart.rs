use hypersig::{NumberFormat, Page, Signals};
use hypersig_ui::DonutChart;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Plan {
    name: String,
    revenue: f64,
}

#[derive(Signals, Serialize, Deserialize)]
struct Revenue {
    plans: Vec<Plan>,
}

const DOLLARS: NumberFormat = NumberFormat::new().prefix("$");

let chart = DonutChart::new("Revenue by plan", Revenue::plans(), Plan::name(), Plan::revenue())
    .format(DOLLARS);

let plans = vec![
    Plan { name: "Hobby".into(), revenue: 1200.0 },
    Plan { name: "Pro".into(), revenue: 5400.0 },
    Plan { name: "Team".into(), revenue: 3400.0 },
];
let html = Page::new("Revenue", Revenue { plans }).child(chart).render();
# assert!(html.contains(">$10,000</span>"));
# assert!(html.contains(r#"stroke-dasharray="54.000 46.000""#));
# assert!(html.contains("<caption>Revenue by plan</caption>"));
