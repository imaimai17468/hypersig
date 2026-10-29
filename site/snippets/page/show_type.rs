use hypersig::html::p;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

// error[E0277]: `Sig<Counter, i32>` is not an expression of type `bool` on
// the page whose signals are `Counter`
let notice = p::<Counter>().show(Counter::count()).text("Counting");
