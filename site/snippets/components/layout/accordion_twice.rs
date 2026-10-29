use hypersig::html::{Id, p};
use hypersig::{Page, Signals};
use hypersig_ui::Accordion;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Help {}

const FAQ: Id = Id::new("faq");

let faq = || Accordion::exclusive(FAQ).item("Is it free?", p().text("Yes."));

// Panics: two elements carry the id `faq`.
let html = Page::new("Help", Help {}).child(faq()).child(faq()).render();
