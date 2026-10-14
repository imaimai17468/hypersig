#![allow(unused)]
use hypersig::html::p;
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
    let _ = p::<Shop>().show(Shop::size().gt(&Size::Small));
}
