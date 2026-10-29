use hypersig::{Page, Signals, if_else};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    unread: i32,
}

let unread = Inbox::unread();
let page = Page::new("Inbox", Inbox { unread: 3 }).bind_title(if_else(
    unread.gt(0),
    unread.to_text().concat(" unread - Inbox"),
    "Inbox",
));
assert!(page.render().contains("<title>3 unread - Inbox</title>"));
