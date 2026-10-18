#![allow(unused)]
use hypersig::{Form, Signals, Validate};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Signup {
    email: String,
}

impl Validate for Signup {}

#[derive(Signals, Serialize, Deserialize)]
struct Login {
    email: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Join {
    signup: Form<Signup>,
}

fn main() {
    let _ = Join::signup().error(Login::email());
}
