use hypersig::{Form, Signals, Validate};
use hypersig_ui::checkbox_in_form;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Signup {
    email: String,
}

impl Validate for Signup {}

#[derive(Signals, Serialize, Deserialize)]
struct Join {
    signup: Form<Signup>,
}

fn main() {
    // A checkbox holds a `bool`, so a text field cannot be its value.
    let _ = checkbox_in_form("Email", Join::signup(), Signup::email());
}
