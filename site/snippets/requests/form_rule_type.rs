use hypersig::{Rules, Sig, Signals, Validate};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Signup {
    password: String,
}

impl Validate for Signup {
    fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
        let password = form.at(Signup::password());
        // error[E0277]: `Expr<S, i32>` is not an expression of type `bool` on the page whose signals are `S`
        rules.check(Signup::password(), password.char_count(), "Use 8 or more characters.");
    }
}
