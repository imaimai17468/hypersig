use hypersig::html::{button, input, span};
use hypersig::{Action, FieldErrors, Form, Page, Patches, Rules, Sig, Signals, Validate};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone, Default)]
struct Signup {
    email: String,
    password: String,
}

// The rules, written once. The browser and `Form::validate` evaluate the same expressions.
impl Validate for Signup {
    fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
        let email = form.at(Signup::email());
        rules
            .check(Signup::email(), email.contains("@"), "Enter an email address.")
            .check(
                Signup::password(),
                form.at(Signup::password()).char_count().ge(8),
                "Use 8 or more characters.",
            );
    }
}

#[derive(Signals, Serialize, Deserialize)]
struct Join {
    signup: Form<Signup>,
}

const JOIN: Action<Join, Signup> = Action::post("/join");

let signup = Join::signup();
let email = Signup::email();
let page = Page::new("Join", Join { signup: Form::new(Signup::default()) }).child(
    signup
        .form(JOIN.call_with(signup.value()))
        .child(input().bind(signup.field(email)).on_blur(signup.touch(email)))
        .child(span().bind_text(signup.error(email)))
        .child(button().show(signup.is_dirty()).on_click(signup.reset()).text("Clear"))
        .child(button().attr("type", "submit").text("Join")),
);

// The handler checks the same rules, then what only the server knows.
async fn join((): (), value: Signup) -> Patches<Join> {
    let signup = Join::signup();
    if let Err(errors) = Form::validate(&value) {
        return signup.reject(&errors).into();
    }
    if value.email == "taken@example.com" {
        let taken = FieldErrors::new().add(Signup::email(), "That address is taken.");
        return signup.reject(&taken).into();
    }
    signup.patch(Form::new(Signup::default())).into()
}
# let _ = (page.render(), join);
