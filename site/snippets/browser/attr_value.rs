use hypersig::html::{attr, input};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Signup {
    email: String,
}

// The name is a constant, and the value is any text: `emial` compiles.
let email = input::<Signup>()
    .attr(attr::TYPE, "emial")
    .attr(hypersig::attr!("data-track"), "signup-email")
    .bind(Signup::email());
# assert!(email.render().starts_with(r#"<input type="emial" data-track="signup-email""#));
