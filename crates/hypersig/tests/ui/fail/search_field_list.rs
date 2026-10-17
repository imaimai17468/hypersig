//! Search parameters with a field that holds a list.

use hypersig::{Search, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Search, Serialize, Deserialize)]
struct Filter {
    q: String,
    tags: Vec<String>,
}

fn main() {}
