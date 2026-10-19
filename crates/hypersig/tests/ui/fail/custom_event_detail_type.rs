#![allow(unused)]

use hypersig::html::{CustomEvent, el};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Map {
    zoom: i32,
    label: String,
}

const ZOOMED: CustomEvent<i32> = CustomEvent::new("zoomed");

fn main() {
    let _ = el::<Map>("x-map").on_custom(ZOOMED, |zoom| Map::label().set(zoom));
}
