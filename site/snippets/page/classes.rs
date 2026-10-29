use hypersig::html::{Class, li};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Todo {
    done: bool,
}

// Declared once, so a misspelled class at a use site is an unknown name.
const ITEM: Class = Class::new("item");
const DONE: Class = Class::new("done");

let html = Page::new("Todo", Todo { done: true })
    .child(li().class(ITEM).class_when(DONE, Todo::done()).text("Buy milk"))
    .render();
assert!(html.contains(r#"<li class="item done""#));
