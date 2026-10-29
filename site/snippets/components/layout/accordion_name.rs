use hypersig::html::p;
use hypersig::Signals;
use hypersig_ui::Accordion;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Help {}

// error[E0308]: mismatched types: expected `Id`, found `&str`
let shipping = Accordion::<Help>::exclusive("shipping")
    .item("How long does delivery take?", p().text("Three to five working days."));
