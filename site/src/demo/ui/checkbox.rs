//! A checkbox and a switch, for the checkbox and switch page.

use hypersig::html::{div, p};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals, if_else};
use hypersig_ui::{checkbox, switch};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Settings {
    terms: bool,
    notifications: bool,
}

pub fn page() -> Page<Settings> {
    let notifications = Settings::notifications();
    let initial = Settings {
        terms: false,
        notifications: true,
    };
    Page::new("Settings", initial).child(
        div()
            .css(stack(rem(1)))
            .child(checkbox("Accept the terms", Settings::terms()))
            .child(switch("Notifications", notifications))
            .child(p().bind_text(if_else(
                notifications,
                "You will get an email for each release.",
                "You will not get emails.",
            ))),
    )
}
