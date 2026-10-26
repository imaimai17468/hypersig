// A path written as a string would skip the page's base, so `script` takes an `Asset`.
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Place {}

fn main() {
    let _ = Page::new("Place", Place {}).script("/map.js");
}
