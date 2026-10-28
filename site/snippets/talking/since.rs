use futures::{Stream, stream};
use hypersig::html::{div, each_keyed, li, p};
use hypersig::{Patches, Signals, StreamAction};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Message {
    id: i32,
    body: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Chat {
    messages: Vec<Message>,
    last_id: i32,
    live: bool,
    problem: String,
}

hypersig::routes! {
    fn pages(Router<()>);
    stream SINCE: StreamAction<Chat, i32> = "/messages" => since;
}

# fn stored_after(_id: i32) -> Vec<Message> { Vec::new() }
// Each batch appends what came after `last_id` and moves `last_id` past it.
fn since((): (), last_id: i32) -> impl Stream<Item = Patches<Chat>> {
    let missed = stored_after(last_id);
    let newest = missed.last().map_or(last_id, |message| message.id);
    stream::iter([Patches::new()
        .list(Chat::messages().patch_append(&missed))
        .signals(Chat::last_id().patch(newest))])
}

// Each attempt to open the stream reads `last_id` again.
let open = SINCE
    .call_with(Chat::last_id())
    .reconnect()
    .connected(Chat::live())
    .error(Chat::problem());
let chat = div::<Chat>()
    .on_load(open)
    .child(p().show(!Chat::live()).bind_text(Chat::problem()))
    .child(each_keyed(
        Chat::messages(),
        |message| message.at(Message::id()),
        |message| li().bind_text(message.at(Message::body())),
    ));
# let _ = (chat, pages);
