use hypersig::html::div;
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

// error[E0277]: `Sig<Settings, bool>` is not an expression of type `bool` on the page whose signals are `Counter`
let panel = div::<Counter>().show(Settings::dark());
