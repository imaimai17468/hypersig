use hypersig::Signals;
use hypersig_ui::radio_group;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Plan {
    Free,
    Pro,
}

#[derive(Signals, Serialize, Deserialize)]
struct Billing {
    plan: Plan,
}

// The options' values written as text, where the signal holds a `Plan`.
let plans = radio_group("Plan", Billing::plan(), [("free", "Free"), ("pro", "Pro")]);
# let _ = plans;
