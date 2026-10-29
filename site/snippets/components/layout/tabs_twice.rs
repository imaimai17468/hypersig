use hypersig::html::p;
use hypersig::{Page, Segment, Signals};
use hypersig_ui::Tabs;
use serde::{Deserialize, Serialize};

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Section {
    Account,
    Password,
}

#[derive(Signals, Serialize, Deserialize)]
struct Settings {
    section: Section,
}

let tabs = || {
    Tabs::new("Settings", Settings::section(), |section| match section {
        Section::Account => ("Account", p().text("Your name and email.")),
        Section::Password => ("Password", p().text("Your password.")),
    })
};

// Panics: two elements carry the id of the first panel.
let html = Page::new("Settings", Settings { section: Section::Account })
    .child(tabs())
    .child(tabs())
    .render();
