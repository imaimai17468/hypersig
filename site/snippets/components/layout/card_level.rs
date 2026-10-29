use hypersig::Signals;
use hypersig_ui::{Level, card_title};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Pricing {}

// error[E0599]: no variant, associated function, or constant named `H1` found
//     for enum `Level`
let title = card_title::<Pricing>(Level::H1).text("Pro");
