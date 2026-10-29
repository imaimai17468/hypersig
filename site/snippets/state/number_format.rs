use hypersig::html::span;
use hypersig::{NumberFormat, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Invoice {
    cents: i64,
    rate: f64,
}

const DOLLARS: NumberFormat = NumberFormat::new().digits(2).prefix("$");
const PERCENT: NumberFormat = NumberFormat::new().digits(1).suffix("%");

// `write` gives the text the browser writes, so a test can check it.
assert_eq!(DOLLARS.write(-1234.5), "-$1,234.50");
assert_eq!(PERCENT.write(12.25), "12.3%");

let amount = span::<Invoice>().bind_text((Invoice::cents().to_f64() / 100.0).format(&DOLLARS));
let rate = span::<Invoice>().bind_text((Invoice::rate() * 100.0).format(&PERCENT));
# let _ = (amount, rate);
