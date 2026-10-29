use hypersig::html::{Class, div};
use hypersig::{Asset, Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Legacy {
    open: bool,
}

// The page links the file. Nothing reads it, so the class names are
// unchecked against it.
const LEGACY_CSS: Asset = Asset::new("/legacy.css");
const DRAWER: Class = Class::new("drawer");
const OPEN: Class = Class::new("is-open");

let html = Page::new("Legacy", Legacy { open: false })
    .stylesheet(LEGACY_CSS)
    .child(div().class(DRAWER).class_when(OPEN, Legacy::open()))
    .render();
assert!(html.contains(r#"<link rel="stylesheet" href="/legacy.css">"#));
