//! Search parameters with a list of lists and an optional list, neither of
//! which has a form in a query string of repeated keys.

use hypersig::{Search, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Search, Serialize, Deserialize)]
struct Filter {
    q: String,
    tags: Vec<String>,
    groups: Vec<Vec<String>>,
    colors: Option<Vec<String>>,
}

fn main() {}
