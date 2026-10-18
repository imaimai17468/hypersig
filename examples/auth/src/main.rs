//! Signing in with a cookie session. An extractor reads the cookie and finds
//! the signed-in member, the sign-in action sets the cookie beside its
//! patches, and an action whose handler fails answers with a status the page
//! shows.
//!
//! Run with `cargo run -p example-auth`, open <http://localhost:3007>, and
//! sign in as `ada` with the password `lovelace`.

use std::collections::HashMap;
use std::hash::{BuildHasher as _, Hasher as _, RandomState};
use std::sync::{Arc, Mutex, MutexGuard};

use axum::Router;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderName, StatusCode, header};
use axum::response::{IntoResponse, Redirect, Response};
use hypersig::html::{El, div, each, form, h1, li, p, span, ul};
use hypersig::style::Style;
use hypersig::style::pattern::{container, hstack, stack};
use hypersig::style::value::rem;
use hypersig::{
    Action, ActionResponse, FieldErrors, Form, Page, Patches, Route, RouterExt, Rules, Sig,
    Signals, Validate, if_else,
};
use hypersig_ui::{
    Autocomplete, ButtonSize, ButtonVariant, InputKind, Level, TextField, button, card,
    card_content, card_header, card_title, submit_button, token,
};
use serde::{Deserialize, Serialize};

/// How many notes a member keeps.
const MAX_NOTES: usize = 3;

#[derive(Signals, Serialize, Deserialize, Clone, Default)]
struct Credentials {
    name: String,
    password: String,
}

impl Validate for Credentials {
    fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
        rules
            .check(
                Credentials::name(),
                !form.at(Credentials::name()).trim().is_empty(),
                "Enter your name.",
            )
            .check(
                Credentials::password(),
                !form.at(Credentials::password()).is_empty(),
                "Enter your password.",
            );
    }
}

#[derive(Signals, Serialize, Deserialize)]
struct Login {
    credentials: Form<Credentials>,
    signing_in: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Home {
    name: String,
    notes: Vec<String>,
    draft: String,
    saving: bool,
    problem: String,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const HOME: Route = "/" => home;
    const LOGIN: Route = "/login" => login;
}

const SIGN_IN: Action<Login, Credentials> = Action::post("/sign-in");
const SIGN_OUT: Action<Home> = Action::post("/sign-out");
const ADD_NOTE: Action<Home, String> = Action::post("/notes");

#[derive(Clone)]
struct AppState {
    passwords: Arc<HashMap<&'static str, &'static str>>,
    /// The member each session token belongs to.
    sessions: Arc<Mutex<HashMap<String, String>>>,
    notes: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl AppState {
    fn sessions(&self) -> MutexGuard<'_, HashMap<String, String>> {
        self.sessions
            .lock()
            .expect("no handler panics while holding the lock")
    }

    fn notes_of(&self, name: &str) -> Vec<String> {
        self.notes
            .lock()
            .expect("no handler panics while holding the lock")
            .get(name)
            .cloned()
            .unwrap_or_default()
    }

    fn add_note(&self, name: &str, note: String) -> Result<Vec<String>, NotesFull> {
        let mut notes = self
            .notes
            .lock()
            .expect("no handler panics while holding the lock");
        let kept = notes.entry(name.to_owned()).or_default();
        if kept.len() >= MAX_NOTES {
            return Err(NotesFull);
        }
        kept.push(note);
        Ok(kept.clone())
    }
}

/// A member's notes are full. It answers with a 409, which the page's
/// `Call::error` shows.
struct NotesFull;

impl IntoResponse for NotesFull {
    fn into_response(self) -> Response {
        let message = format!("You can keep {MAX_NOTES} notes.");
        (StatusCode::CONFLICT, message).into_response()
    }
}

/// The member whose session cookie came with the request.
struct Member {
    name: String,
    token: String,
}

impl FromRequestParts<AppState> for Member {
    type Rejection = SignedOut;

    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl Future<Output = Result<Self, SignedOut>> + Send {
        std::future::ready(Self::read(parts, state))
    }
}

impl Member {
    fn read(parts: &Parts, state: &AppState) -> Result<Self, SignedOut> {
        let token = parts
            .headers
            .get_all(header::COOKIE)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .flat_map(|cookies| cookies.split(';'))
            .find_map(|cookie| cookie.trim().strip_prefix("session="))
            .map(str::to_owned);
        let name = token
            .as_ref()
            .and_then(|token| state.sessions().get(token).cloned());
        match (token, name) {
            (Some(token), Some(name)) => Ok(Self { name, token }),
            _ => Err(SignedOut {
                from_action: parts
                    .headers
                    .get(header::ACCEPT)
                    .is_some_and(|accept| accept.as_bytes() == b"text/event-stream"),
            }),
        }
    }
}

/// A request without a live session.
///
/// A page sends the browser to the sign-in page. An action answers with a
/// 401 instead, because `fetch` would follow a redirect and hand the page
/// the sign-in page's HTML, which it cannot apply.
struct SignedOut {
    from_action: bool,
}

impl IntoResponse for SignedOut {
    fn into_response(self) -> Response {
        if self.from_action {
            let message = "Your session has ended. Sign in again.";
            (StatusCode::UNAUTHORIZED, message).into_response()
        } else {
            Redirect::to(LOGIN.url(()).as_str()).into_response()
        }
    }
}

/// A new session token.
///
/// `RandomState` is seeded from the operating system's random source, so
/// two hashes of nothing give 128 unpredictable bits. An application would
/// use a crate made for tokens.
fn new_token() -> String {
    let half = || RandomState::new().build_hasher().finish();
    format!("{:016x}{:016x}", half(), half())
}

/// The `Set-Cookie` header that stores `token`, or removes the cookie when
/// `token` is `None`.
fn session_cookie(token: Option<&str>) -> [(HeaderName, String); 1] {
    let value = match token {
        Some(token) => format!("session={token}; HttpOnly; SameSite=Lax; Path=/"),
        None => "session=; Max-Age=0; HttpOnly; SameSite=Lax; Path=/".to_owned(),
    };
    [(header::SET_COOKIE, value)]
}

async fn home(state: AppState, member: Member, (): ()) -> Page<Home> {
    let signals = Home {
        notes: state.notes_of(&member.name),
        name: member.name,
        draft: String::new(),
        saving: false,
        problem: String::new(),
    };
    Page::new("hypersig notes", signals)
        .theme(hypersig_ui::theme())
        .child(frame(home_card()))
}

async fn login(_state: AppState, (): ()) -> Page<Login> {
    let signals = Login {
        credentials: Form::new(Credentials::default()),
        signing_in: false,
    };
    Page::new("Sign in", signals)
        .theme(hypersig_ui::theme())
        .child(frame(login_card()))
}

fn frame<S>(content: El<S>) -> El<S> {
    div()
        .css(
            container(rem(28), rem(1))
                .padding_block(rem(3))
                .merge(stack(rem(1.5))),
        )
        .child(h1().css(Style::new().margin(rem(0))).text("Notes"))
        .child(content)
}

fn login_card() -> El<Login> {
    let credentials = Login::credentials();
    let signing_in = Login::signing_in();
    card().child(
        credentials
            .form(SIGN_IN.call_with(credentials.value()).indicator(signing_in))
            .css(stack(rem(1.5)))
            .child(card_header().child(card_title(Level::H2).text("Sign in")))
            .child(
                card_content()
                    .css(stack(rem(1)))
                    .child(
                        TextField::in_form("Name", credentials, Credentials::name())
                            .autocomplete(Autocomplete::Username)
                            .description("Try ada with the password lovelace."),
                    )
                    .child(
                        TextField::in_form("Password", credentials, Credentials::password())
                            .kind(InputKind::Password)
                            .autocomplete(Autocomplete::CurrentPassword),
                    )
                    .child(
                        submit_button(ButtonVariant::Default, ButtonSize::Default)
                            .attr_when("disabled", signing_in)
                            .bind_text(if_else(signing_in, "Signing in…", "Sign in")),
                    ),
            ),
    )
}

fn home_card() -> El<Home> {
    let draft = Home::draft();
    let saving = Home::saving();
    let muted = Style::new().color(token::muted_foreground().var());
    card().child(
        div()
            .css(stack(rem(1.5)))
            .child(
                card_header().child(
                    div()
                        .css(hstack(rem(0.75)))
                        .child(
                            p().id("member")
                                .css(muted.clone())
                                .text("Signed in as ")
                                .child(span().bind_text(Home::name())),
                        )
                        .child(
                            button(ButtonVariant::Outline, ButtonSize::Sm)
                                .on_click(SIGN_OUT.call())
                                .text("Sign out"),
                        ),
                ),
            )
            .child(
                card_content()
                    .css(stack(rem(1)))
                    .child(
                        ul().id("notes")
                            .css(stack(rem(0.5)))
                            .child(each(Home::notes(), |note| li().bind_text(note.value()))),
                    )
                    .child(
                        p().css(muted)
                            .show(Home::notes().is_empty())
                            .text("No notes yet."),
                    )
                    .child(
                        form()
                            .css(stack(rem(0.75)))
                            .on_submit(
                                ADD_NOTE
                                    .call_with(draft)
                                    .indicator(saving)
                                    .error(Home::problem()),
                            )
                            .child(TextField::new("Note", draft))
                            .child(
                                submit_button(ButtonVariant::Default, ButtonSize::Default)
                                    .attr_when("disabled", saving)
                                    .text("Add note"),
                            )
                            .child(
                                p().id("problem")
                                    .attr("role", "alert")
                                    .css(Style::new().color(token::destructive().var()))
                                    .bind_text(Home::problem()),
                            ),
                    ),
            ),
    )
}

async fn sign_in(state: AppState, credentials: Credentials) -> ActionResponse<Login> {
    let form = Login::credentials();
    if let Err(errors) = Form::validate(&credentials) {
        return form.reject(&errors).into();
    }
    let name = credentials.name.trim();
    if state.passwords.get(name) != Some(&credentials.password.as_str()) {
        let wrong = FieldErrors::new().add(
            Credentials::password(),
            "The name or the password is wrong.",
        );
        return form.reject(&wrong).into();
    }
    let token = new_token();
    state.sessions().insert(token.clone(), name.to_owned());
    // The cookie rides on the same response as the patch that sends the
    // browser home, so the home page's request already carries it.
    let reply = Patches::new().navigate(HOME.url(()));
    (session_cookie(Some(&token)), reply).into()
}

async fn sign_out(
    state: AppState,
    member: Member,
    (): (),
) -> ([(HeaderName, String); 1], Patches<Home>) {
    state.sessions().remove(&member.token);
    (session_cookie(None), Patches::new().navigate(LOGIN.url(())))
}

async fn add_note(
    state: AppState,
    member: Member,
    note: String,
) -> Result<Patches<Home>, NotesFull> {
    let note = note.trim();
    if note.is_empty() {
        return Ok(Patches::new());
    }
    let notes = state.add_note(&member.name, note.to_owned())?;
    Ok(Home::notes()
        .patch(notes)
        .and(Home::draft().patch(String::new()))
        .into())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = AppState {
        passwords: Arc::new(HashMap::from([("ada", "lovelace"), ("grace", "hopper")])),
        sessions: Arc::default(),
        notes: Arc::default(),
    };
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(SIGN_IN, sign_in)
        .action(SIGN_OUT, sign_out)
        .action(ADD_NOTE, add_note)
        .with_state(state);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3007".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}
