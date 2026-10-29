use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

// error[E0599]: no associated function or constant named `cuont` found for struct `Counter`
let count = Counter::cuont();
