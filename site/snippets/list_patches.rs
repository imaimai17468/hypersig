use hypersig::{Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Message {
    id: i32,
    body: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Room {
    messages: Vec<Message>,
    oldest: i32,
}

let messages = Room::messages();
let by_id = |message: hypersig::Item<Room, Message>| message.at(Message::id());
let incoming = Message { id: 42, body: "hello".to_owned() };

// One new message, as a stream's next event.
let new: Patches<Room> = messages.patch_append(&[incoming]).into();

// An older page goes above the messages on screen, which stay where they are.
let older = vec![Message { id: 30, body: "earlier".to_owned() }];
let page: Patches<Room> = Patches::new()
    .list(messages.patch_prepend(&older))
    .signals(Room::oldest().patch(30));

// An edit replaces the message with its id, and a deletion removes it.
let edited = Message { id: 42, body: "hello again".to_owned() };
let edit: Patches<Room> = messages
    .patch_upsert(&[edited], by_id)
    .and(messages.patch_remove(&[7], by_id))
    .into();

assert_eq!(new.items_of(messages)[0].body, "hello");
# let _ = (page, edit);
