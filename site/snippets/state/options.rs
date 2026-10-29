use hypersig::html::{button, div, span};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    name: String,
    nickname: Option<String>,
}

let name = Profile::name();
let nickname = Profile::nickname();

let card = div::<Profile>()
    .child(span().bind_text(nickname.unwrap_or(name)))
    .child(span().show(nickname.is_none()).text(" (no nickname)"))
    // `&None` sets the field back to `None`.
    .child(button().on_click(nickname.set(&None)).text("Clear"))
    .child(button().on_click(nickname.set(name.some())).text("Use my name"));
# let html = Page::new("Profile", Profile { name: "Ada".into(), nickname: None }).child(card).render();
# assert!(html.contains(">Ada</span>"), "{html}");
