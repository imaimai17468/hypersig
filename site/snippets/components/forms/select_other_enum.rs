use hypersig::Signals;
use hypersig_ui::select_field;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Region {
    Tokyo,
    Osaka,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Plan {
    Free,
    Pro,
}

#[derive(Signals, Serialize, Deserialize)]
struct Deploy {
    region: Region,
}

// The options of another enum.
let region = select_field("Region", Deploy::region(), [(Plan::Free, "Free"), (Plan::Pro, "Pro")]);
# let _ = region;
