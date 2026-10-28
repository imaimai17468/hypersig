use hypersig::html::button;
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    action ADD: Action<Counter> = post "/add" => |(), ()| async { Patches::<Counter>::new() };
}

// error[E0599]: no method named `reconnect` found for struct `Call<S>` in the current scope
let add = button::<Counter>().on_click(ADD.call().reconnect());
# let _ = (add, pages);
