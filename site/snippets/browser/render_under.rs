use hypersig::html::p;
use hypersig::{Asset, Base, Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Notes {}

// A GitHub Pages project site is served under the repository's name.
const SITE: Base = Base::new("/hypersig");
const RUNTIME: Asset = Asset::new("/hypersig.js");
const STYLE: Asset = Asset::new("/style.css");

let page = Page::new("Notes", Notes {})
    .runtime_src(RUNTIME)
    .stylesheet(STYLE)
    .child(p().text("Hello"));
let html = page.render_under(SITE);
# assert!(html.contains(r#"src="/hypersig/hypersig.js""#), "{html}");
# assert!(html.contains(r#"href="/hypersig/style.css""#), "{html}");
// Write `html` to `notes/index.html`, and `RUNTIME_JS` to `hypersig.js`.
