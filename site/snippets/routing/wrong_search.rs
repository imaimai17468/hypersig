# use serde::{Deserialize, Serialize};
use hypersig::{Search, Signals};

#[derive(Signals, Serialize, Deserialize)]
struct Range {
    min: i32,
    max: i32,
}

// error[E0277]: `Range` cannot be a field of search parameters
#[derive(Signals, Search, Serialize, Deserialize)]
struct Filter {
    price: Range,
}
