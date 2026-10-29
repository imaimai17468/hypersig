use hypersig::{Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    unread: i32,
}

// error[E0308]: mismatched types: expected `i32`, found `String`
let patches: Patches<Inbox> = Inbox::unread().patch("3 new".to_owned()).into();
