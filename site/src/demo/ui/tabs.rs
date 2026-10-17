//! A tab list, for the tabs page.

use hypersig::html::p;
use hypersig::{Page, Signals};
use hypersig_ui::Tabs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Section {
    Account,
    Password,
    Billing,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Settings {
    section: Section,
}

pub fn page() -> Page<Settings> {
    let initial = Settings {
        section: Section::Account,
    };
    Page::new("Settings", initial).child(
        Tabs::new("Settings", Settings::section())
            .tab(
                Section::Account,
                "Account",
                p().text("Change your name and email."),
            )
            .tab(
                Section::Password,
                "Password",
                p().text("Passwords are at least twelve characters."),
            )
            .tab(
                Section::Billing,
                "Billing",
                p().text("Invoices arrive on the first of each month."),
            ),
    )
}
