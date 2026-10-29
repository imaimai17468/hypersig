use hypersig::Signals;
use hypersig_ui::{ButtonVariant, icon_button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {}

// An icon button with no name for a screen reader to announce.
let more = icon_button::<App>(ButtonVariant::Ghost).text("…");
# let _ = more;
