use hypersig::Signals;
use hypersig_ui::spinner;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Sync {}

// A status without a name, which a screen reader has nothing to read for.
let busy = spinner::<Sync>();
