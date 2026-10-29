use hypersig::html::{div, each, li, ul};
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, checkbox, submit_button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Task {
    title: String,
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Signup {
    terms: bool,
    tasks: Vec<Task>,
}

let initial = Signup {
    terms: false,
    tasks: vec![Task { title: "Read the guide".into(), done: true }],
};
let page = Page::new("Sign up", initial).child(
    div()
        .child(checkbox("I accept the terms", Signup::terms()))
        // A checkbox needs no id, so it works on a field of a list item.
        .child(ul().child(each(Signup::tasks(), |task| {
            li().child(checkbox("Done", task.at(Task::done())))
        })))
        .child(
            submit_button(ButtonVariant::Default, ButtonSize::Default)
                .disabled(!Signup::terms())
                .text("Continue"),
        ),
);
# let html = page.render();
# assert!(html.contains(r#"type="checkbox""#));
