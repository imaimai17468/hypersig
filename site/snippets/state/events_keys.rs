use hypersig::html::{Key, div, input, textarea};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Editor {
    title: String,
    body: String,
    saved: bool,
}

let title = Editor::title();

let editor = div::<Editor>()
    // Ctrl+S, or Command+S on a Mac. A plain `s` still types in the fields.
    .on_key(Key::Char('s').primary(), Editor::saved().set(true))
    .child(input().bind(title).on_key(Key::Escape, title.set("")))
    .child(textarea().bind(Editor::body()));
# let html = Page::new("Editor", Editor { title: String::new(), body: String::new(), saved: false }).child(editor).render();
# let _ = html;
