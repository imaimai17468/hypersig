//! Search parameters that are not a `#[derive(Signals)]` struct.

use hypersig::Route;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Filter {
    page: i32,
}

const FIND: Route<(), Filter> = Route::new("/find");

fn main() {
    let _ = FIND.path();
}
