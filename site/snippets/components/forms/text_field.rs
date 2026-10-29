use hypersig::html::div;
use hypersig::{Page, Signals, if_else};
use hypersig_ui::{Autocomplete, InputKind, InputMode, TextField};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Signup {
    email: String,
    code: String,
    bio: String,
}

let email = Signup::email();
let looks_wrong = !email.is_empty().or(email.contains("@"));
let initial = Signup { email: String::new(), code: String::new(), bio: String::new() };
let page = Page::new("Sign up", initial).child(
    div()
        .child(
            TextField::new("Email", email)
                .kind(InputKind::Email)
                .autocomplete(Autocomplete::Email)
                .placeholder("ada@example.com")
                .description("We send the receipt here.")
                // Empty while the address looks right, which keeps the field valid.
                .error(if_else(looks_wrong, "Enter an email address.", ""))
                .required(),
        )
        .child(
            TextField::new("Code", Signup::code())
                .input_mode(InputMode::Numeric)
                .autocomplete(Autocomplete::OneTimeCode)
                .min_length(6)
                .max_length(6)
                .disabled(email.is_empty()),
        )
        .child(
            TextField::new("Bio", Signup::bio())
                .multiline()
                .max_length(280)
                .description("A sentence or two for your profile."),
        ),
);
# let html = page.render();
# assert!(html.contains(r#"type="email" autocomplete="email""#));
# assert!(html.contains(r#"aria-invalid="false""#));
# assert!(html.contains("<textarea"));
