use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

// A plain string, where an `Asset` belongs.
let page = Page::new("Counter", Counter { count: 0 }).runtime_src("./hypersig.js");
# let _ = page;
