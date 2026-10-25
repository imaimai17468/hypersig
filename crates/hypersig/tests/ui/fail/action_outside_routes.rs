//! Actions, streams, and uploads declared outside `routes!`, which would have no handler.

use hypersig::{Action, Signals, StreamAction, Upload};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

const ADD: Action<Counter> = Action::post("/add");
const TICKS: StreamAction<Counter> = StreamAction::new("/ticks");
const PICTURE: Upload<Counter> = Upload::post("/picture");

fn main() {
    let _ = (ADD, TICKS, PICTURE);
}
