# use hypersig::{Action, FieldErrors, Form, Patches, Rules, Sig, Signals, Validate};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize, Clone, Default)]
# struct Signup {
#     email: String,
#     password: String,
# }
# impl Validate for Signup {
#     fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
#         let email = form.at(Signup::email());
#         rules.check(Signup::email(), email.contains("@"), "Enter an email address.");
#     }
# }
# #[derive(Signals, Serialize, Deserialize)]
# struct Join {
#     signup: Form<Signup>,
# }
# hypersig::routes! {
#     fn pages(Router<()>);
#     action JOIN: Action<Join, Signup> = post "/join" => join;
# }
# let _ = pages;
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
    // Saved: these values are the form's new starting values.
    signup.patch(Form::new(value)).into()
}
