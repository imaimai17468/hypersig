#![allow(unused)]
use hypersig::html::input;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
enum Size {
    Small,
    Large,
}

#[derive(Signals, Serialize, Deserialize)]
struct Shop {
    size: Size,
}

fn main() {
    let _ = input::<Shop>().bind(Shop::size());
}
