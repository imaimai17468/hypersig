use hypersig::html::{Key, div};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Docs {
    help: bool,
}

let help = Docs::help();

let page = Page::new("Docs", Docs { help: false })
    // Runs wherever focus is, and while nothing has focus.
    .on_key(Key::Char('?'), help.toggle())
    .on_key(Key::Escape, help.set(false))
    .child(div().show(help).text("Press ? or Escape to close this help."));
# let _ = page.render();
