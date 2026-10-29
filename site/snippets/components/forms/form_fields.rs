use hypersig::{Action, CalendarDate, Form, Page, Rules, Sig, Signals, Validate};
use hypersig_ui::{
    ButtonSize, ButtonVariant, DateField, InputKind, NumberField, TextField, checkbox_in_form,
    radio_group_in_form, select_field_in_form, submit_button, switch_in_form,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
enum Plan {
    #[default]
    Unset,
    Hobby,
    Pro,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
enum Region {
    #[default]
    Tokyo,
    Frankfurt,
}

#[derive(Signals, Serialize, Deserialize, Clone, Default)]
struct Signup {
    email: String,
    seats: i32,
    start: Option<CalendarDate>,
    plan: Plan,
    region: Region,
    newsletter: bool,
    terms: bool,
}

impl Validate for Signup {
    fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
        let email = form.at(Signup::email());
        rules.check(Signup::email(), email.contains("@"), "Enter an email address.");
        rules.check(Signup::seats(), form.at(Signup::seats()).ge(1), "Add at least one seat.");
        let chosen = !form.at(Signup::plan()).equals(&Plan::Unset);
        rules.check(Signup::plan(), chosen, "Choose a plan.");
        rules.check(Signup::terms(), form.at(Signup::terms()), "Accept the terms.");
    }
}

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
let page = Page::new("Join", Join { signup: Form::new(Signup::default()) }).child(
    signup
        .form(JOIN.call_with(signup.value()))
        .child(TextField::in_form("Email", signup, Signup::email()).kind(InputKind::Email))
        .child(NumberField::in_form("Seats", signup, Signup::seats()).min(1))
        .child(DateField::in_form("Start", signup, Signup::start()))
        .child(radio_group_in_form(
            "Plan",
            signup,
            Signup::plan(),
            [(Plan::Hobby, "Hobby"), (Plan::Pro, "Pro")],
        ))
        .child(select_field_in_form(
            "Region",
            signup,
            Signup::region(),
            [(Region::Tokyo, "Tokyo"), (Region::Frankfurt, "Frankfurt")],
        ))
        .child(switch_in_form("Newsletter", signup, Signup::newsletter()))
        .child(checkbox_in_form("I accept the terms", signup, Signup::terms()))
        .child(submit_button(ButtonVariant::Default, ButtonSize::Default).text("Join")),
);
# let html = page.render();
# assert!(html.contains(r#"role="radiogroup""#));
# assert!(html.contains(r#"type="submit""#));
# let _ = pages;
