//! Signing in with a cookie session kept by `tower-sessions`. Its layer reads
//! the cookie, loads the session, and sets the cookie after a handler changes
//! the session. An extractor finds the signed-in member in the session, and an
//! action whose handler fails answers with a status the page shows. Passwords
//! are kept as Argon2 hashes, which `password-auth` writes and checks. A
//! member uploads a picture through a file input, which an upload checks
//! before it keeps it.
//!
//! Run with `cargo run -p example-auth`, open <http://localhost:3007>, and
//! sign in as `ada` with the password `lovelace`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use axum::Router;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
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
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use tower_sessions::cookie::SameSite;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};

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
    /// The status of the last failed note, or `0`.
    failure: i32,
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
    /// Each member's password hash, as `password_auth::generate_hash` wrote
    /// it. An application reads these from its database.
    password_hashes: Arc<HashMap<&'static str, &'static str>>,
    notes: Arc<Mutex<HashMap<String, Vec<String>>>>,
    /// Each member's picture, described as the page shows it.
    pictures: Arc<Mutex<HashMap<String, String>>>,
}

impl AppState {
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

/// A hash of a password no member has, which a sign-in with an unknown name
/// is checked against.
const NO_MEMBER_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$DA82QPNeuglt9UgI4H53EQ$uBZf8usNhSnVATPWCcGBXpsznx7CjIxRbE0NGwp+kMQ";

/// The key under which a session holds the signed-in member's name.
const MEMBER: &str = "member";

/// The member whose session cookie came with the request.
struct Member {
    name: String,
    session: Session,
}

impl FromRequestParts<AppState> for Member {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Response> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(IntoResponse::into_response)?;
        match session.get::<String>(MEMBER).await {
            Ok(Some(name)) => Ok(Self { name, session }),
            Ok(None) => Err(SignedOut {
                kind: RequestKind::of(parts),
            }
            .into_response()),
            Err(error) => Err(StoreFailed(error).into_response()),
        }
    }
}

/// A request without a signed-in member.
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

/// The session store failed to read or write a session. It answers with a
/// 500, which an action's `Call::error` shows.
struct StoreFailed(tower_sessions::session::Error);

impl IntoResponse for StoreFailed {
    fn into_response(self) -> Response {
        eprintln!("session store: {}", self.0);
        let message = "Your session could not be read. Try again.";
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
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
        failure: 0,
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
                                    .error(Home::problem())
                                    .status(Home::failure()),
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
                            )
                            .child(
                                a().id("sign-in-again")
                                    .show(signed_out())
                                    .href(LOGIN.link())
                                    .text("Sign in again"),
                            ),
                    )
                    .child(picture_form()),
            ),
    )
}

/// Whether the last note failed because the session had ended, which the
/// status of `SignedOut`'s answer says.
fn signed_out() -> hypersig::Expr<Home, bool> {
    Home::failure().equals(i32::from(StatusCode::UNAUTHORIZED.as_u16()))
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

async fn sign_in(
    state: AppState,
    session: Session,
    credentials: Credentials,
) -> Result<ActionResponse<Login>, StoreFailed> {
    let form = Login::credentials();
    if let Err(errors) = Form::validate(&credentials) {
        return Ok(form.reject(&errors).into());
    }
    let name = credentials.name.trim().to_owned();
    let member = state.password_hashes.get(name.as_str()).copied();
    // A name no member has is checked against a hash as well, so the answer
    // takes as long as a member's and does not tell which names exist.
    let hash = member.unwrap_or(NO_MEMBER_HASH);
    let password = credentials.password;
    // Argon2 spends its time on the CPU on purpose, about 10 ms in a release
    // build, so it runs on tokio's blocking threads, not the ones that serve
    // the other requests.
    let verified = tokio::task::spawn_blocking(move || verify_password(password, hash).is_ok())
        .await
        .expect("verifying a password does not panic");
    if member.is_none() || !verified {
        let wrong = FieldErrors::new().add(
            Credentials::password(),
            "The name or the password is wrong.",
        );
        return Ok(form.reject(&wrong).into());
    }
    // A new id for the session that now holds a member. Someone who placed
    // the id of their own session in this browser before the sign-in still
    // holds the old id, and the store no longer has it.
    session.cycle_id().await.map_err(StoreFailed)?;
    session.insert(MEMBER, name).await.map_err(StoreFailed)?;
    // The layer sets the cookie on this response, the one whose patch sends
    // the browser home, so the home page's request already carries it.
    Ok(Patches::new().navigate(HOME.url(())).into())
}

async fn sign_out(_state: AppState, member: Member, (): ()) -> Result<Patches<Home>, StoreFailed> {
    // An empty session makes the layer remove the cookie.
    member.session.flush().await.map_err(StoreFailed)?;
    Ok(Patches::new().navigate(LOGIN.url(())))
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

/// The layer that keeps sessions, in this process's memory.
///
/// `MemoryStore` is lost on a restart and is not shared with a second
/// process, so a deployment with more than one instance passes a store
/// that keeps sessions in a database instead.
fn sessions() -> SessionManagerLayer<MemoryStore> {
    SessionManagerLayer::new(MemoryStore::default())
        .with_name("session")
        // The default, `Strict`, leaves the cookie off a visit that follows a
        // link from another site, which would land on the sign-in page. An
        // action needs no `Strict`, because it refuses a request another
        // site sends.
        .with_same_site(SameSite::Lax)
        // The default too. Chrome stores a `Secure` cookie from
        // `http://localhost` and `http://127.0.0.1` as well as over HTTPS. A
        // browser that does not drops it, and the sign-in then lands back on
        // the sign-in page.
        .with_secure(true)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = AppState {
        // The hashes of `lovelace` and `hopper`.
        password_hashes: Arc::new(HashMap::from([
            (
                "ada",
                "$argon2id$v=19$m=19456,t=2,p=1$Ezh4pqRXf0ii8dWn2w4iIw$Lm5DFxXxtEIh7WeT2RRVzLM20Fdb1azMfaHbwyabOI4",
            ),
            (
                "grace",
                "$argon2id$v=19$m=19456,t=2,p=1$/rzCQklh7Wj8paOo76GUNQ$mf4BTOP+5oG2wLmxJiLNAdTtPYpiWfnSktpmdEbHoWA",
            ),
        ])),
        notes: Arc::default(),
        pictures: Arc::default(),
    };
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(SIGN_IN, sign_in)
        .action(SIGN_OUT, sign_out)
        .action(ADD_NOTE, add_note)
        .upload(UPLOAD_PICTURE, upload_picture)
        .with_state(state)
        .layer(sessions());
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3007".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}
