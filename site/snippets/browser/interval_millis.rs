use hypersig::html::div;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Clock {
    tenths: i32,
}

let tenths = Clock::tenths();
// A number of milliseconds where a `Duration` belongs.
let clock = div::<Clock>().on_interval(100, tenths.set(tenths + 1));
# let _ = clock;
