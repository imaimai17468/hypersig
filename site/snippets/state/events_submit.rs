use hypersig::html::{attr, button, form, input};
use hypersig::{Page, Signals, Stmt};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Tags {
    draft: String,
    tags: Vec<String>,
}

let draft = Tags::draft();
let add = Tags::tags().push(draft.trim()).then(draft.set(""));

// Enter in the field and a click on the button both submit the form.
let new_tag = form::<Tags>()
    .on_submit(Stmt::when(!draft.trim().is_empty(), add))
    .child(input().bind(draft))
    .child(button().attr(attr::TYPE, "submit").text("Add"));
# let html = Page::new("Tags", Tags { draft: String::new(), tags: Vec::new() }).child(new_tag).render();
# let _ = html;
