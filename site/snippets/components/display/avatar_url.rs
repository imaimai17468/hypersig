use hypersig::Signals;
use hypersig_ui::Avatar;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Team {}

let user_input = String::from("javascript:alert(1)");
// A string is not a checked URL.
let ada = Avatar::<Team>::new("Ada Lovelace", "AL").picture(user_input.as_str());
