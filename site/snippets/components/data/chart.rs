use hypersig::{NumberFormat, Page, Signals};
use hypersig_ui::{Chart, token};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Day {
    date: String,
    signups: i32,
    trials: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Growth {
    days: Vec<Day>,
}

const PEOPLE: NumberFormat = NumberFormat::new().suffix(" people");

let chart = Chart::area("Signups and trials", Growth::days(), Day::date())
    .series("Signups", Day::signups(), token::chart_1())
    .series("Trials", Day::trials(), token::chart_2())
    .format(PEOPLE)
    // Thirty days, labeled every third day along the bottom.
    .max_labels(10);

let days = (1..=30)
    .map(|day| Day { date: format!("Jun {day}"), signups: 1200 + day * 35, trials: 300 + day * 4 })
    .collect();
let html = Page::new("Growth", Growth { days }).child(chart).render();
# assert!(html.contains("<caption>Signups and trials</caption>"));
# assert!(html.contains(r#"<th scope="row">Jun 30</th><td>2,250 people</td>"#));
# assert!(html.contains(r#"preserveAspectRatio="none""#));
