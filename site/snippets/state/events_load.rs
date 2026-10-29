use hypersig::html::{article, button, div, p, when};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Story {
    comments_open: bool,
    times_opened: i32,
    read_to_end: bool,
}

let opened = Story::times_opened();

let story = article::<Story>()
    .child(button().on_click(Story::comments_open().toggle()).text("Comments"))
    // `when` adds the element each time the condition turns true,
    // so `on_load` runs on each opening.
    .child(when(
        Story::comments_open(),
        div().on_load(opened.set(opened + 1)).text("No comments yet."),
    ))
    // Runs each time the last paragraph scrolls into view.
    .child(p().on_visible(Story::read_to_end().set(true)).text("The end."));
# let html = Page::new("Story", Story { comments_open: false, times_opened: 0, read_to_end: false }).child(story).render();
# let _ = html;
