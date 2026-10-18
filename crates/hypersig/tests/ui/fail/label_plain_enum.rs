//! A label on an enum without `#[derive(Segment)]`, whose variants nothing lists.
#![allow(unused)]
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Status {
    Open,
    Closed,
}

#[derive(Signals, Serialize, Deserialize)]
struct Ticket {
    status: Status,
}

fn main() {
    let _ = Ticket::status().label(|status| match status {
        Status::Open => "Open",
        Status::Closed => "Closed",
    });
}
