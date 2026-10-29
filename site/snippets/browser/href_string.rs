use hypersig::html::a;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    name: String,
}

// A plain string, where a link or a `SafeUrl` belongs.
let source = a::<Profile>().href("https://github.com/imaimai17468/hypersig").text("Source");
# let _ = source;
