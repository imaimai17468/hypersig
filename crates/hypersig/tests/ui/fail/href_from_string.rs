#![allow(unused)]

use hypersig::Signals;
use hypersig::html::a;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    website: String,
}

fn main() {
    // A visitor's text could be a `javascript:` URL, so it becomes an
    // `href` only through `SafeUrl::parse`.
    let _ = a::<Profile>().href(Profile::website());
}
