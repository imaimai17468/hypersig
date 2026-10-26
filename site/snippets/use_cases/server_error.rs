# use hypersig::html::{attr, button, input, span};
# use hypersig::{Action, FieldErrors, Form, Page, Patches, Rules, Sig, Signals, Validate};
# use serde::{Deserialize, Serialize};
#
# #[derive(Signals, Serialize, Deserialize, Clone, Default)]
# struct Signup {
#     email: String,
# }
#
# impl Validate for Signup {
#     fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
#         let email = form.at(Signup::email());
#         rules.check(Signup::email(), email.contains("@"), "Enter an email address.");
#     }
# }
#
# #[derive(Signals, Serialize, Deserialize)]
# struct Join {
#     signup: Form<Signup>,
# }
#
# #[derive(Clone)]
# struct AppState;
#
# impl AppState {
#     fn is_taken(&self, email: &str) -> bool {
#         email == "taken@example.com"
#     }
# }
#
# hypersig::routes! {
#     fn pages(Router<AppState>);
#     action JOIN: Action<Join, Signup> = post "/join" => join;
# }
#
# let signup = Join::signup();
# let email = Signup::email();
# let page = Page::new("Join", Join { signup: Form::new(Signup::default()) }).child(
#     signup
#         .form(JOIN.call_with(signup.value()))
#         .child(input().bind(signup.field(email)).on_blur(signup.touch(email)))
// Under the field, the message the browser's rules write.
.child(span().bind_text(signup.error(email)))
#         .child(button().attr(attr::TYPE, "submit").text("Join")),
# );

async fn join(state: AppState, value: Signup) -> Patches<Join> {
    let signup = Join::signup();
    if let Err(errors) = Form::validate(&value) {
        return signup.reject(&errors).into();
    }
    if state.is_taken(&value.email) { //+
        let taken = FieldErrors::new().add(Signup::email(), "That address is taken."); //+
        return signup.reject(&taken).into(); //+
    } //+
    signup.patch(Form::new(Signup::default())).into()
}
# let _ = (page.render(), pages);
