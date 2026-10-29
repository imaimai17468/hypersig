use hypersig::Signals;
use hypersig_ui::Listbox;
use serde::{Deserialize, Serialize};

// No `#[derive(Segment)]`, so the listbox cannot list the variants.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Plan {
    Hobby,
    Pro,
}

#[derive(Signals, Serialize, Deserialize)]
struct Billing {
    plan: Plan,
}

let plans = Listbox::new("Plan", Billing::plan(), |plan| match plan {
    Plan::Hobby => "Hobby",
    Plan::Pro => "Pro",
});
# let _ = plans;
