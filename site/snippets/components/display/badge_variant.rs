use hypersig::Signals;
use hypersig_ui::badge;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Build {}

// The look is a `BadgeVariant`, not a string.
let passing = badge::<Build>("secondary").text("Passing");
