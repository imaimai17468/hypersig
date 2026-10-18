//! Signing in with a cookie session. An extractor reads the cookie and finds
//! the signed-in member, the sign-in action sets the cookie beside its
//! patches, and an action whose handler fails answers with a status the page
//! shows. A member uploads a picture through a file input, which an upload
//! checks before it keeps it.
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
use axum::response::{IntoResponse, Response};
use hypersig::html::{El, a, div, each, form, h1, label, li, p, span, ul};
use hypersig::style::Style;
use hypersig::style::pattern::{container, hstack, stack};
use hypersig::style::value::rem;
use hypersig::{
    Action, ActionResponse, FieldErrors, FileInput, Form, Page, PageResponse, Patches, RequestKind,
    Route, RouterExt, Rules, Sig, Signals, Unserved, Upload, Uploaded, Validate, if_else,
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
    /// What the member's picture is, such as `cat.png, 1024 bytes`, or `""`.
    picture: String,
    uploading: bool,
    picture_problem: String,
}

hypersig::routes! {
    fn pages(Router<AppState>) else missing;
    const HOME: Route = "/" => home;
    const LOGIN: Route = "/login" => login;
}

const SIGN_IN: Action<Login, Credentials> = Action::post("/sign-in");
const SIGN_OUT: Action<Home> = Action::post("/sign-out");
const ADD_NOTE: Action<Home, String> = Action::post("/notes");
const PICTURE_FILE: FileInput<Home> = FileInput::new("picture");
const UPLOAD_PICTURE: Upload<Home> = Upload::post("/picture").max_bytes(64 * 1024);

#[derive(Clone)]
struct AppState {
    passwords: Arc<HashMap<&'static str, &'static str>>,
    /// The member each session token belongs to.
    sessions: Arc<Mutex<HashMap<String, String>>>,
    notes: Arc<Mutex<HashMap<String, Vec<String>>>>,
    /// Each member's picture, described as the page shows it.
    pictures: Arc<Mutex<HashMap<String, String>>>,
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

    fn pictures(&self) -> MutexGuard<'_, HashMap<String, String>> {
        self.pictures
            .lock()
            .expect("no handler panics while holding the lock")
    }
}

/// An upload that is not one PNG or JPEG picture. It answers with a 422.
struct NotAPicture;

impl IntoResponse for NotAPicture {
    fn into_response(self) -> Response {
        let message = "Choose one PNG or JPEG picture.";
        (StatusCode::UNPROCESSABLE_ENTITY, message).into_response()
    }
}

/// Whether `bytes` start as a PNG or a JPEG file does. The media type the
/// browser sends comes from the file name, so the bytes decide.
fn is_picture(bytes: &[u8]) -> bool {
    bytes.starts_with(b"\x89PNG\r\n\x1a\n") || bytes.starts_with(&[0xff, 0xd8, 0xff])
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
                kind: RequestKind::of(parts),
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
    kind: RequestKind,
}

impl IntoResponse for SignedOut {
    fn into_response(self) -> Response {
        match self.kind {
            RequestKind::Runtime => {
                let message = "Your session has ended. Sign in again.";
                (StatusCode::UNAUTHORIZED, message).into_response()
            }
            RequestKind::Navigation => PageResponse::redirect(LOGIN.url(())).into_response(),
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
        picture: state
            .pictures()
            .get(&member.name)
            .cloned()
            .unwrap_or_default(),
        name: member.name,
        draft: String::new(),
        saving: false,
        problem: String::new(),
        uploading: false,
        picture_problem: String::new(),
    };
    Page::new("hypersig notes", signals)
        .theme(hypersig_ui::theme())
        .child(frame(home_card()))
}

/// The page for a path that names no page, sent with a 404.
#[derive(Signals, Serialize, Deserialize)]
struct Missing {
    problem: String,
}

async fn missing(_state: AppState, unserved: Unserved) -> Page<Missing> {
    let signals = Missing {
        problem: unserved.to_string(),
    };
    Page::new("Not found", signals)
        .theme(hypersig_ui::theme())
        .child(frame(
            card().child(
                card_content()
                    .css(stack(rem(1)))
                    .child(p().id("missing").text("There is no page here."))
                    .child(
                        p().css(Style::new().color(token::muted_foreground().var()))
                            .bind_text(Missing::problem()),
                    )
                    .child(a().href(HOME.link()).text("Back to your notes")),
            ),
        ))
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
                    )
                    .child(picture_form()),
            ),
    )
}

fn picture_form() -> El<Home> {
    let uploading = Home::uploading();
    let picture = Home::picture();
    let muted = Style::new().color(token::muted_foreground().var());
    div()
        .css(stack(rem(0.75)))
        .child(label().attr("for", "picture-file").text("Picture"))
        .child(
            PICTURE_FILE
                .input()
                .id("picture-file")
                .attr("accept", "image/png,image/jpeg"),
        )
        .child(
            p().id("picture")
                .css(muted.clone())
                .show(!picture.is_empty())
                .text("Your picture: ")
                .child(span().bind_text(picture)),
        )
        .child(
            button(ButtonVariant::Default, ButtonSize::Default)
                .attr_when("disabled", uploading)
                .on_click(
                    UPLOAD_PICTURE
                        .call(PICTURE_FILE)
                        .indicator(uploading)
                        .error(Home::picture_problem())
                        .then(PICTURE_FILE.clear()),
                )
                .bind_text(if_else(uploading, "Uploading…", "Upload picture")),
        )
        .child(
            p().id("picture-problem")
                .attr("role", "alert")
                .css(Style::new().color(token::destructive().var()))
                .bind_text(Home::picture_problem()),
        )
        .child(
            p().css(muted.font_size(rem(0.875)))
                .text("PNG or JPEG, up to 64 KiB."),
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

async fn upload_picture(
    state: AppState,
    member: Member,
    uploaded: Uploaded<()>,
) -> Result<Patches<Home>, NotAPicture> {
    let [file] = uploaded.files() else {
        return Err(NotAPicture);
    };
    if !is_picture(file.bytes()) {
        return Err(NotAPicture);
    }
    let described = format!(
        "{}, {} bytes",
        file.file_name().unwrap_or("picture"),
        file.bytes().len()
    );
    state.pictures().insert(member.name, described.clone());
    Ok(Home::picture().patch(described).into())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = AppState {
        passwords: Arc::new(HashMap::from([("ada", "lovelace"), ("grace", "hopper")])),
        sessions: Arc::default(),
        notes: Arc::default(),
        pictures: Arc::default(),
    };
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(SIGN_IN, sign_in)
        .action(SIGN_OUT, sign_out)
        .action(ADD_NOTE, add_note)
        .upload(UPLOAD_PICTURE, upload_picture)
        .with_state(state);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3007".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}
