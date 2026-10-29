use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    unread: i32,
}

// error[E0277]: `Sig<Inbox, i32>` is not an expression of type `String` on
// the page whose signals are `Inbox`
let page = Page::new("Inbox", Inbox { unread: 3 }).bind_title(Inbox::unread());
