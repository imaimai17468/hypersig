use hypersig::{Form, Signals, Validate};
use hypersig_ui::select_field_in_form;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Signup {
    plan: String,
}

impl Validate for Signup {}

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Login {
    plan: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Join {
    signup: Form<Signup>,
}

fn main() {
    // The field belongs to `Login`, not to the form's `Signup`.
    let _ = select_field_in_form(
        "Plan",
        Join::signup(),
        Login::plan(),
        [("free".to_owned(), "Free")],
    );
}
