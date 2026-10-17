//! A text field with a description and an error, for the text field page.

use hypersig::html::div;
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals, if_else};
use hypersig_ui::{Autocomplete, InputKind, TextField};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Signup {
    email: String,
    bio: String,
}

pub fn page() -> Page<Signup> {
    let email = Signup::email();
    let looks_wrong = !email.is_empty().or(email.contains("@"));
    let initial = Signup {
        email: String::new(),
        bio: String::new(),
    };
    Page::new("Sign up", initial).child(
        div()
            .css(stack(rem(1.25)).max_width(rem(24)))
            .child(
                TextField::new("Email", email)
                    .kind(InputKind::Email)
                    .autocomplete(Autocomplete::Email)
                    .placeholder("ada@example.com")
                    .description("We send the receipt here.")
                    .error(if_else(looks_wrong, "Enter an email address.", ""))
                    .required(),
            )
            .child(
                TextField::new("Bio", Signup::bio())
                    .multiline()
                    .description("A sentence or two for your profile."),
            ),
    )
}
