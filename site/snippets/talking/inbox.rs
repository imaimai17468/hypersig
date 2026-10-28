use std::sync::{Arc, Mutex};

use hypersig::html::{button, each_keyed, li, p, span, ul};
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Message {
    id: i32,
    subject: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    messages: Vec<Message>,
    unread: i32,
    status: String,
}

#[derive(Clone, Default)]
struct Store(Arc<Mutex<Vec<Message>>>);

hypersig::routes! {
    fn pages(Router<Store>);
    action ARCHIVE: Action<Inbox, i32> = post "/archive" => archive;
}

// One answer removes the message and sets two signals.
async fn archive(store: Store, id: i32) -> Patches<Inbox> {
    let mut messages = store.0.lock().expect("no handler panics while it holds the store");
    messages.retain(|message| message.id != id);
    let left = i32::try_from(messages.len()).unwrap_or(i32::MAX);
    Patches::new()
        .list(Inbox::messages().patch_remove(&[id], |message| message.at(Message::id())))
        .signals(
            Inbox::unread()
                .patch(left)
                .and(Inbox::status().patch(format!("{left} left"))),
        )
}

let page = ul::<Inbox>()
    .child(each_keyed(
        Inbox::messages(),
        |message| message.at(Message::id()),
        |message| {
            li().child(span().bind_text(message.at(Message::subject())))
                .child(button().on_click(ARCHIVE.call_with(message.at(Message::id()))).text("Archive"))
        },
    ))
    .child(p().bind_text(Inbox::status()));

# tokio::runtime::Runtime::new().unwrap().block_on(async {
let store = Store::default();
store.0.lock().unwrap().push(Message { id: 7, subject: "Hello".to_owned() });
let patches = archive(store, 7).await;
assert_eq!(patches.value_of(Inbox::unread()), Some(0));
assert_eq!(patches.value_of(Inbox::status()), Some("0 left".to_owned()));
assert_eq!(patches.removed_keys_of(Inbox::messages(), |message| message.at(Message::id())), [7]);
# });
# let _ = (page, pages);
