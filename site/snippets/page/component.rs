use hypersig::html::{El, input, label, span};
use hypersig::{Sig, Signals};
use serde::{Deserialize, Serialize};

/// A labeled text field bound to `value`, on any page.
fn field<S>(text: &str, value: Sig<S, String>) -> El<S> {
    label()
        .child(span().text(text))
        .child(input().bind(value))
}

#[derive(Signals, Serialize, Deserialize)]
struct Signup {
    name: String,
    email: String,
}

let form = hypersig::html::form::<Signup>()
    .child(field("Name", Signup::name()))
    .child(field("Email", Signup::email()));
# assert!(form.render().contains("<input"));
