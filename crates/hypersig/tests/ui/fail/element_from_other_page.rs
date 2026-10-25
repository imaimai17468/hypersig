//! An element built for one page's signals, placed inside an element of another page.

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

fn main() {
    let _ = div::<Detail>().child(badge());
}
