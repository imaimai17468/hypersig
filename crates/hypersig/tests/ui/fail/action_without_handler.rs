//! An action in `routes!` that names no handler.

use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    action ADD: Action<Counter> = post "/add";
}

fn main() {}
