use hypersig::html::{div, p};
use hypersig::{Page, Signals, if_else};
use hypersig_ui::radio_group;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Plan {
    Hobby,
    Pro,
    Team,
}

#[derive(Signals, Serialize, Deserialize)]
struct Billing {
    plan: Plan,
}

let plan = Billing::plan();
let page = Page::new("Billing", Billing { plan: Plan::Hobby }).child(
    div()
        .child(radio_group(
            "Plan",
            plan,
            [(Plan::Hobby, "Hobby"), (Plan::Pro, "Pro"), (Plan::Team, "Team")],
        ))
        .child(p().bind_text(if_else(plan.equals(&Plan::Hobby), "Free forever.", "Billed monthly."))),
);
# let html = page.render();
# assert!(html.contains("<fieldset"));
# assert_eq!(html.matches(r#"type="radio""#).count(), 3);
