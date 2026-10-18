use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    unread: i32,
}

fn main() {
    let _ = Page::new("Inbox", Inbox { unread: 3 }).bind_title(Inbox::unread());
}
