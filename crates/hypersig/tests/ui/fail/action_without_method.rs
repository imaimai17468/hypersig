//! An action in `routes!` that names no method before its path.

use hypersig::{Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    action ADD: Action<Counter> = "/add" => add;
}

async fn add((): (), (): ()) -> Patches<Counter> {
    Counter::count().patch(1).into()
}

fn main() {
    let _ = pages;
}
