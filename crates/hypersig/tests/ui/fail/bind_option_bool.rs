//! A checkbox bound to an `Option<bool>`, which has no empty state to stand for `None`.
#![allow(unused)]
use hypersig::html::input;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Survey {
    agreed: Option<bool>,
}

fn main() {
    let _ = input::<Survey>().bind(Survey::agreed());
}
