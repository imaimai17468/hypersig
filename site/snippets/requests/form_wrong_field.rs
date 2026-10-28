use hypersig::html::input;
use hypersig::{Form, Signals, Validate};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Signup {
    email: String,
}

impl Validate for Signup {}

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Profile {
    email: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Join {
    signup: Form<Signup>,
}

let signup = Join::signup();
// error[E0308]: mismatched types
// expected `Sig<Signup, _>`, found `Sig<Profile, String>`
let _email = input().bind(signup.field(Profile::email()));
