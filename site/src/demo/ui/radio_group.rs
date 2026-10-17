//! A radio group, for the radio group page.

use hypersig::html::{div, p};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals, if_else};
use hypersig_ui::radio_group;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Plan {
    Hobby,
    Pro,
    Team,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Billing {
    plan: Plan,
}

pub fn page() -> Page<Billing> {
    let plan = Billing::plan();
    Page::new("Billing", Billing { plan: Plan::Pro }).child(
        div()
            .css(stack(rem(1)))
            .child(radio_group(
                "Plan",
                plan,
                [
                    (Plan::Hobby, "Hobby"),
                    (Plan::Pro, "Pro"),
                    (Plan::Team, "Team"),
                ],
            ))
            .child(p().bind_text(if_else(
                plan.equals(&Plan::Hobby),
                "Free for personal projects.",
                "Billed monthly.",
            ))),
    )
}
