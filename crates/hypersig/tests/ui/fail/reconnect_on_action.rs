use hypersig::html::button;
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

hypersig::routes! {
    fn actions(Router<()>);
    action ADD: Action<Counter> = post "/add" => |(), ()| async { Patches::<Counter>::new() };
}

fn main() {
    // Sending a POST again could add twice, so only a `StreamCall` reconnects.
    let _ = button::<Counter>().on_click(ADD.call().reconnect());
}
