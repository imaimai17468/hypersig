use hypersig::Signals;
use hypersig::html::p;
use hypersig_ui::Tabs;
use serde::{Deserialize, Serialize};

// Without `#[derive(Segment)]`, nothing lists the variants, so a tab could
// be left out.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Section {
    Account,
    Password,
}

#[derive(Signals, Serialize, Deserialize)]
struct Settings {
    section: Section,
}

fn main() {
    let _ = Tabs::new("Settings", Settings::section(), |section| match section {
        Section::Account => ("Account", p().text("Your name.")),
        Section::Password => ("Password", p().text("Your password.")),
    });
}
