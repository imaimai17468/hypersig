use hypersig::html::p;
use hypersig::{Segment, Signals};
use hypersig_ui::Tabs;
use serde::{Deserialize, Serialize};

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Section {
    Account,
    Password,
    Billing,
}

#[derive(Signals, Serialize, Deserialize)]
struct Settings {
    section: Section,
}

// A selected section without a tab would leave no tab that Tab reaches, so
// the `match` names a tab for every variant.
fn main() {
    let _ = Tabs::new("Settings", Settings::section(), |section| match section {
        Section::Account => ("Account", p().text("Your name.")),
        Section::Password => ("Password", p().text("Your password.")),
    });
}
