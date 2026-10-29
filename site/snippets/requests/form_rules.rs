use hypersig::{Form, Rules, Sig, Signals, Validate};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone, Default)]
struct Signup {
    email: String,
    password: String,
    confirm: String,
}

impl Validate for Signup {
    fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
        let email = form.at(Signup::email());
        let password = form.at(Signup::password());
        rules
            // Two checks on one field: the first that fails gives the message.
            .check(Signup::email(), !email.is_empty(), "Enter an email address.")
            .check(Signup::email(), email.contains("@"), "That is not an email address.")
            .check(Signup::password(), password.char_count().ge(8), "Use 8 or more characters.")
            // A rule can read the other fields.
            .check(
                Signup::confirm(),
                form.at(Signup::confirm()).equals(password),
                "The passwords differ.",
            );
    }
}

// The server evaluates the same expressions.
let sent = Signup { email: "ada".into(), password: "short".into(), confirm: "other".into() };
let errors = Form::validate(&sent).unwrap_err();
assert_eq!(errors.message(Signup::email()), Some("That is not an email address."));
assert_eq!(errors.message(Signup::confirm()), Some("The passwords differ."));
