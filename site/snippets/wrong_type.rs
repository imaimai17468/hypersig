use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

// error[E0277]: `&str` is not an expression of type `i32` on the page whose signals are `Counter`
let reset = Counter::count().set("zero");
