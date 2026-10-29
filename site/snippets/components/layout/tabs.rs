use hypersig::html::p;
use hypersig::{Page, Segment, Signals};
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

let tabs = Tabs::new("Settings", Settings::section(), |section| match section {
    Section::Account => ("Account", p().text("Change your name and email.")),
    Section::Password => ("Password", p().text("Passwords are at least twelve characters.")),
    Section::Billing => ("Billing", p().text("Invoices arrive on the first of each month.")),
});

// The page opens on the tab its first value names.
let html = Page::new("Settings", Settings { section: Section::Password })
    .child(tabs)
    .render();
# assert!(html.contains(r#"role="tablist" aria-label="Settings""#));
# assert!(html.contains(r#"aria-selected="true" tabindex="0""#));
# assert_eq!(html.matches(r#"role="tabpanel""#).count(), 3);
# assert!(html.contains(r#">Password</button>"#));
