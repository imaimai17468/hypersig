use hypersig::Signals;
use hypersig_ui::Toast;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {
    messages: Vec<String>,
}

// A toast goes into a list of toasts.
let save = Toast::new("Saved").push(App::messages());
