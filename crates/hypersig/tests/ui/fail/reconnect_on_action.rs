use hypersig::html::button;
use hypersig::{Action, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

const ADD: Action<Counter> = Action::post("/add");

fn main() {
    // Sending a POST again could add twice, so only a `StreamCall` reconnects.
    let _ = button::<Counter>().on_click(ADD.call().reconnect());
}
