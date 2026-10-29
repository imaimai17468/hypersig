use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::url;
use hypersig::{Base, Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Note {}

const APP: Base = Base::new("/app");

let page = Page::new("Note", Note {}).child(
    div().css(Style::new().background_image(url("/paper.png"))),
);
// A path from the root goes under the base the page is served under.
assert!(page.render_under(APP).contains(r#"url("/app/paper.png")"#));
assert!(page.render().contains(r#"url("/paper.png")"#));
