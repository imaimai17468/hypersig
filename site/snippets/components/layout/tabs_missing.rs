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

// error[E0004]: non-exhaustive patterns: `&Section::Billing` not covered
let tabs = Tabs::new("Settings", Settings::section(), |section| match section {
    Section::Account => ("Account", p().text("Change your name and email.")),
    Section::Password => ("Password", p().text("Change your password.")),
});
