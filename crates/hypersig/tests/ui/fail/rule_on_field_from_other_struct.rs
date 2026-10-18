#![allow(unused)]
use hypersig::{Rules, Sig, Signals, Validate};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Signup {
    email: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Login {
    email: String,
}

impl Validate for Signup {
    fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
        rules.check(Login::email(), form.at(Signup::email()).contains("@"), "Enter an email address.");
    }
}

fn main() {}
