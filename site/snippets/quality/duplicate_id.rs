use hypersig::html::{Id, div, input, label};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Signup {
    email: String,
    backup: String,
}

const EMAIL: Id = Id::new("email");

let page = Page::new("Sign up", Signup { email: String::new(), backup: String::new() }).child(
    div()
        .child(label().label_for(&EMAIL).text("Email"))
        .child(input().id(EMAIL).bind(Signup::email()))
        // A second field copied from the first still carries its id.
        .child(input().id(EMAIL).bind(Signup::backup())),
);
// panicked: two elements carry the id `email` where the runtime can reach
// both, so its statements act on whichever comes first: give one of them
// another name
let _ = page.render();
