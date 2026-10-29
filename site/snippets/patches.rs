use hypersig::{Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    unread: i32,
    status: String,
}

// One signal, as a handler's whole answer.
let one: Patches<Inbox> = Inbox::unread().patch(0).into();

// Several signals in one event.
let both: Patches<Inbox> = Inbox::unread()
    .patch(3)
    .and(Inbox::status().patch("3 new messages".to_owned()))
    .into();

assert_eq!(both.value_of(Inbox::unread()), Some(3));
assert_eq!(one.value_of(Inbox::status()), None);
