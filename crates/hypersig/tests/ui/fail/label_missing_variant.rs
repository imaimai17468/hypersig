//! A label mapping whose `match` leaves out a variant.
#![allow(unused)]
use hypersig::{Segment, Signals};
use serde::{Deserialize, Serialize};

#[derive(Segment, Serialize, Deserialize)]
enum Status {
    Open,
    InReview,
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
