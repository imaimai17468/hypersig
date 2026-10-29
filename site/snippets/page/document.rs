use hypersig::html::h1;
use hypersig::{IntlLocale, Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    count: i32,
}

let page = Page::new("Notes", Notes { count: 2 })
    .lang(IntlLocale::En)
    .child(h1().text("Notes"));
let html = page.render();
assert!(html.starts_with(r#"<!doctype html><html lang="en"><head>"#));
assert!(html.contains("<title>Notes</title>"));
assert!(html.contains(hypersig::RUNTIME_PATH));
assert!(html.ends_with("<h1>Notes</h1></body></html>"));
