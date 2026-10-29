use hypersig::{Form, Rules, Sig, Signals, Validate};
use hypersig_ui::TextField;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Signup {
    email: String,
}

impl Validate for Signup {
    fn rules<S>(_form: Sig<S, Self>, _rules: &mut Rules<S, Self>) {}
}

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Profile {
    email: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Join {
    signup: Form<Signup>,
}

// A field of `Profile`, which the form on this page does not hold.
let email = TextField::in_form("Email", Join::signup(), Profile::email());
# let _ = email;
