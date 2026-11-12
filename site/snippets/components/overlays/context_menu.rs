use hypersig::html::{Role, each, li, p, ul};
use hypersig::{NodeRef, Page, Signals};
use hypersig_ui::{ContextMenu, MenuItem};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone, PartialEq)]
struct Message {
    id: i32,
    text: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Chat {
    messages: Vec<Message>,
    picked: i32,
    status: String,
}

hypersig::node_ref!(const MESSAGE_ACTIONS: NodeRef<Chat>);
const MESSAGE_MENU: ContextMenu<Chat> = ContextMenu::new(MESSAGE_ACTIONS);
# fn main() {

let picked = Chat::picked();
let status = Chat::status();
let chat = Chat {
    messages: vec![Message { id: 1, text: "The build is green again.".to_owned() }],
    picked: 0,
    status: String::new(),
};
let html = Page::new("Chat", chat)
    .child(ul().child(each(Chat::messages(), |message| {
        li().bind_text(message.at(Message::text())).on_context_menu(move |at| {
            picked.set(message.at(Message::id())).then(MESSAGE_MENU.open_at(at))
        })
    })))
    .child(MESSAGE_MENU.menu(
        "Message",
        [
            MenuItem::new("Reply", status.set("Replying.")),
            MenuItem::new("Delete", status.set("Deleted the message.")).destructive(),
        ],
    ))
    .child(p().role(Role::Status).bind_text(status))
    .render();
# assert!(html.contains("data-hs-on-contextmenu"));
# assert!(html.contains(r#"role="menu" aria-label="Message""#));
# }
