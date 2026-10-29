use hypersig::Signals;
use hypersig_ui::toggle;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    unread: i32,
    #[signals(computed)]
    has_unread: bool,
}

// A computed signal, which the page derives and a click cannot set.
let filter = toggle(Inbox::has_unread()).text("Unread");
# let _ = filter;
