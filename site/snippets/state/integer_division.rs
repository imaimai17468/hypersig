use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

// error[E0369]: cannot divide `Sig<Counter, i32>` by `{integer}`
let half = Counter::count() / 2;
