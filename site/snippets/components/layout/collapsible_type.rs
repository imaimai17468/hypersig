use hypersig::Signals;
use hypersig_ui::Collapsible;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Releases {
    latest: String,
}

// error[E0308]: mismatched types: expected `Sig<_, bool>`, found `Sig<Releases, String>`
let older = Collapsible::new(Releases::latest());
