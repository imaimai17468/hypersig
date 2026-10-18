#![allow(unused)]
use hypersig::html::{div, part};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Badge {
    count: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Shop {
    badge: Badge,
}

fn main() {
    // `Badge` has no `Part` impl, so there is nothing to render at the field.
    let _ = div::<Shop>().child(part(Shop::badge()));
}
