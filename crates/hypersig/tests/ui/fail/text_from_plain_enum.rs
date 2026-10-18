//! An enum without `#[derive(Segment)]` written as text, whose serde form may not be a string.
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
    let _ = p::<Shop>().bind_text(Shop::size().to_text());
}
