use hypersig::{Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Fruit {
    query: String,
}

fn main() {
    let _ = Patches::<Fruit>::new().push_url("/fruit?q=app");
}
