use hypersig::Signals;
use hypersig_ui::{ButtonSize, button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {}

// The variant written as text.
let save = button::<App>("outline", ButtonSize::Sm).text("Save");
# let _ = save;
