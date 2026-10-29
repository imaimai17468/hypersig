use hypersig::Signals;
use hypersig_ui::{Toast, Toaster};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {
    toasts: Vec<Toast>,
}

// Is 10 ten seconds or ten milliseconds? A `Duration` says which.
let toaster = Toaster::new(App::toasts()).dismiss_after(10);
