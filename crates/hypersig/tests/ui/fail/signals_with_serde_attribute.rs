//! `#[serde(...)]` on a field of a `Signals` struct, whose functions are then used.

use hypersig::html::{div, input};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {
    #[serde(rename = "q")]
    query: String,
    count: i32,
}

fn main() {
    let shop = Shop {
        query: String::new(),
        count: 0,
    };
    let _ = Page::new("Shop", shop).child(
        div()
            .child(input().bind(Shop::query()))
            .bind_text(Shop::count().to_text()),
    );
}
