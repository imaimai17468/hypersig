use hypersig::html::input;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
enum Sort {
    Newest,
    Cheapest,
}

#[derive(Signals, Serialize, Deserialize)]
struct Catalog {
    sort: Sort,
}

// error[E0277]: a form control cannot be bound to a `Sort` signal
let field = input::<Catalog>().bind(Catalog::sort());
