use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {}

// The page-wide rules passed where the dark values of the tokens belong.
let page = Page::new("App", App {}).theme(hypersig_ui::global());
# let _ = page;
