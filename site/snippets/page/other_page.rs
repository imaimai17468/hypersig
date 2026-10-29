use hypersig::html::{El, div, span};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shelf {
    count: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Detail {
    title: String,
}

fn badge() -> El<Shelf> {
    span().bind_text(Shelf::count().to_text())
}

// error[E0277]: `El<Shelf>` is not a child of an element on the page whose
// signals are `Detail`
let detail = div::<Detail>().child(badge());
