use hypersig::html::{attr, button, input, span};
use hypersig::{Action, Form, Page, Rules, Sig, Signals, Validate};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone, Default)]
struct Signup {
    email: String,
    password: String,
}

impl Validate for Signup {
    fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
        let email = form.at(Signup::email());
        rules.check(Signup::email(), email.contains("@"), "Enter an email address.");
    }
}

// The page's signal struct holds the form as a field of type `Form<T>`.
#[derive(Signals, Serialize, Deserialize)]
struct Join {
    signup: Form<Signup>,
}

hypersig::routes! {
    fn pages(Router<()>);
    action JOIN: Action<Join, Signup> = post "/join" => join;
}
# async fn join((): (), _: Signup) -> hypersig::Patches<Join> { hypersig::Patches::new() }

let signup = Join::signup();
let email = Signup::email();
let page = Page::new("Join", Join { signup: Form::new(Signup::default()) }).child(
    signup
        .form(JOIN.call_with(signup.value()))
        .child(input().bind(signup.field(email)).on_blur(signup.touch(email)))
        .child(span().bind_text(signup.error(email)))
        // A `<button>` in a `<form>` submits it unless its type says otherwise.
        .child(
            button()
                .attr(attr::TYPE, "button")
                .show(signup.is_dirty())
                .on_click(signup.reset())
                .text("Clear"),
        )
        .child(button().attr(attr::TYPE, "submit").text("Join")),
);
# let _ = (page.render(), pages);
