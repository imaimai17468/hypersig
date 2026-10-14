#![allow(unused)]

use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
enum Mode {
    Light,
    Dark,
}

fn main() {}
