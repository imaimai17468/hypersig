//! A sign-up form whose rules the browser checks as the visitor types and
//! the server checks again, with the server adding the one rule only it can
//! check: whether the email address is already registered.
//!
//! Run with `cargo run -p example-signup` and open <http://localhost:3006>.

use std::sync::{Arc, Mutex};

use axum::Router;
use hypersig::html::{El, div, h1, p, span};
use hypersig::style::Style;
use hypersig::style::pattern::{container, hstack, stack};
use hypersig::style::value::rem;
use hypersig::{
    Action, FieldErrors, Form, Page, Patches, Route, RouterExt, Rules, Sig, Signals, Validate,
    if_else,
};
use hypersig_ui::{
    Autocomplete, ButtonSize, ButtonVariant, InputKind, Level, TextField, button, card,
    card_content, card_description, card_footer, card_header, card_title, radio_group_in_form,
    submit_button, token,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
enum Plan {
    #[default]
    Unset,
    Personal,
    Team,
}

#[derive(Signals, Serialize, Deserialize, Clone, Default)]
struct Signup {
    name: String,
    email: String,
    password: String,
    confirm: String,
    plan: Plan,
}

impl Validate for Signup {
    fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
        let email = form.at(Signup::email());
        let password = form.at(Signup::password());
        rules
            .check(
                Signup::name(),
                !form.at(Signup::name()).trim().is_empty(),
                "Enter your name.",
            )
            .check(
                Signup::email(),
                !email.is_empty(),
                "Enter an email address.",
            )
            .check(
                Signup::email(),
                email.contains("@"),
                "An email address has an @ in it.",
            )
            .check(
                Signup::password(),
                password.char_count().ge(8),
                "Use 8 or more characters.",
            )
            .check(
                Signup::confirm(),
                form.at(Signup::confirm()).equals(password),
                "The passwords differ.",
            )
            .check(
                Signup::plan(),
                !form.at(Signup::plan()).equals(&Plan::Unset),
                "Choose a plan.",
            );
    }
}

#[derive(Signals, Serialize, Deserialize)]
struct Join {
    signup: Form<Signup>,
    saving: bool,
    welcome: String,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
}

const JOIN: Action<Join, Signup> = Action::post("/join");

#[derive(Clone)]
struct AppState {
    members: Arc<Mutex<Vec<String>>>,
}

async fn index(_state: AppState, (): ()) -> Page<Join> {
    let signals = Join {
        signup: Form::new(Signup::default()),
        saving: false,
        welcome: String::new(),
    };
    Page::new("hypersig sign-up", signals)
        .theme(hypersig_ui::theme())
        .child(
            div()
                .css(
                    container(rem(28), rem(1))
                        .padding_block(rem(3))
                        .merge(stack(rem(1.5))),
                )
                .child(h1().css(Style::new().margin(rem(0))).text("Sign up"))
                .child(signup()),
        )
}

fn signup() -> El<Join> {
    let signup = Join::signup();
    let saving = Join::saving();
    let muted = Style::new().color(token::muted_foreground().var());
    card().child(
        signup
            .form(JOIN.call_with(signup.value()).indicator(saving))
            .css(stack(rem(1.5)))
            .child(
                card_header()
                    .child(card_title(Level::H2).text("Create an account"))
                    .child(card_description().text(
                        "Messages appear when you leave a field. The server checks the same rules, \
                         and whether the address is taken.",
                    )),
            )
            .child(
                card_content()
                    .css(stack(rem(1)))
                    .child(
                        TextField::in_form("Name", signup, Signup::name())
                            .autocomplete(Autocomplete::Name)
                            .required(),
                    )
                    .child(
                        TextField::in_form("Email", signup, Signup::email())
                            .kind(InputKind::Email)
                            .autocomplete(Autocomplete::Email)
                            .description("taken@example.com is already registered.")
                            .required(),
                    )
                    .child(
                        TextField::in_form("Password", signup, Signup::password())
                            .kind(InputKind::Password)
                            .autocomplete(Autocomplete::NewPassword)
                            .required(),
                    )
                    .child(
                        TextField::in_form("Confirm password", signup, Signup::confirm())
                            .kind(InputKind::Password)
                            .autocomplete(Autocomplete::NewPassword)
                            .required(),
                    )
                    .child(radio_group_in_form(
                        "Plan",
                        signup,
                        Signup::plan(),
                        [(Plan::Personal, "Personal"), (Plan::Team, "Team")],
                    )),
            )
            .child(
                card_footer()
                    .css(hstack(rem(0.75)))
                    .child(
                        submit_button(ButtonVariant::Default, ButtonSize::Default)
                            .attr_when("disabled", saving)
                            .bind_text(if_else(saving, "Creating…", "Create account")),
                    )
                    .child(
                        button(ButtonVariant::Outline, ButtonSize::Default)
                            .show(signup.is_dirty())
                            .on_click(signup.reset())
                            .text("Clear"),
                    )
                    .child(
                        span()
                            .id("unsaved")
                            .css(muted.clone())
                            .show(signup.is_dirty())
                            .text("Unsaved changes"),
                    ),
            )
            .child(
                p().id("welcome")
                    .attr("role", "status")
                    .css(muted.padding_inline(rem(1.5)))
                    .bind_text(Join::welcome()),
            ),
    )
}

async fn join(state: AppState, signup: Signup) -> Patches<Join> {
    let form = Join::signup();
    if let Err(errors) = Form::validate(&signup) {
        return form.reject(&errors).into();
    }
    let mut members = state
        .members
        .lock()
        .expect("no handler panics while holding the lock");
    if members.contains(&signup.email) {
        let taken = FieldErrors::new().add(Signup::email(), "That address is already registered.");
        return form.reject(&taken).into();
    }
    members.push(signup.email);
    form.patch(Form::new(Signup::default()))
        .and(Join::welcome().patch(format!("Welcome, {}.", signup.name.trim())))
        .into()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = AppState {
        members: Arc::new(Mutex::new(vec!["taken@example.com".to_owned()])),
    };
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(JOIN, join)
        .with_state(state);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3006".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}
