use hypersig::Signals;
use hypersig::html::div;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {
    dark: bool,
}

// The page's theme passed where the style of one element belongs.
let preview = div::<App>().css_when(hypersig_ui::theme(), App::dark());
# let _ = preview;
