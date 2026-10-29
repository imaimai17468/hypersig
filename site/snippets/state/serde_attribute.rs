use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    // error: `#[serde(...)]` is not supported on a `Signals` struct, because the signal path must equal the field name
    #[serde(rename = "q")]
    query: String,
}
