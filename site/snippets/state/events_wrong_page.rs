use hypersig::html::button;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Settings {
    dark: bool,
}

// error[E0277]: `Stmt<Settings>` is not a statement on the page whose signals are `Counter`
let toggle = button::<Counter>().on_click(Settings::dark().toggle());
