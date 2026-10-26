use hypersig::Signals;
use hypersig_ui::Popover;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {}

// A name written by hand could match another popover's, so a popover takes a
// `NodeRef` that `node_ref!` names after its constant.
const DETAILS: Popover<Profile> = Popover::new("profile-details");

fn main() {
    let _ = DETAILS.popover();
}
