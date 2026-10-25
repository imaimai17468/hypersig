//! A notes page whose server keeps nothing itself: each page load and each
//! action calls the JSON API of a separate notes service with reqwest, and
//! answers the browser with what that service returned.
//!
//! The browser calls only this server's actions, so the service's address
//! and its token stay on the server. Start the stand-in service with
//! `cargo run -p example-api --bin notes-backend`, then run
//! `cargo run -p example-api` and open <http://localhost:3012>. `API_URL`
//! and `API_TOKEN` name the service and its token, and default to the
//! stand-in's.

use axum::Router;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use hypersig::html::{El, attr, button, div, form, h1, input, li, p, ul};
use hypersig::{Action, Fragment, Page, Patches, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct NotesPage {
    draft: String,
    problem: String,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
}

const ADD: Action<NotesPage, String> = Action::post("/notes");

/// A note as the service's JSON has it.
#[derive(Deserialize)]
struct Note {
    id: u64,
    text: String,
}

#[derive(Serialize)]
struct NewNote<'a> {
    text: &'a str,
}

/// The body of a request the service refused.
#[derive(Deserialize)]
struct Refusal {
    error: String,
}

/// A call to the service that did not return what was asked for.
enum ApiError {
    /// The service answered 422 with a message about the visitor's input,
    /// which the visitor can act on.
    Refused(String),
    /// The service could not be reached, answered another status, or sent a
    /// body that does not parse. The visitor can only try again.
    Unavailable(String),
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        Self::Unavailable(error.to_string())
    }
}

/// The runtime writes this body to the signal given to `Call::error`.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Refused(message) => (StatusCode::UNPROCESSABLE_ENTITY, message).into_response(),
            Self::Unavailable(detail) => {
                eprintln!("notes service: {detail}");
                let message = "The notes service did not answer. Try again.";
                (StatusCode::BAD_GATEWAY, message).into_response()
            }
        }
    }
}

/// The client of the notes service. One `reqwest::Client` keeps a pool of
/// connections, so the router's state holds it and every handler shares it.
#[derive(Clone)]
struct Api {
    client: reqwest::Client,
    base: String,
    token: String,
}

impl Api {
    async fn notes(&self) -> Result<Vec<Note>, ApiError> {
        let response = self
            .client
            .get(format!("{}/notes", self.base))
            .bearer_auth(&self.token)
            .send()
            .await?
            .error_for_status()?;
        Ok(response.json().await?)
    }

    async fn add(&self, text: &str) -> Result<(), ApiError> {
        let response = self
            .client
            .post(format!("{}/notes", self.base))
            .bearer_auth(&self.token)
            .json(&NewNote { text })
            .send()
            .await?;
        if response.status() == StatusCode::UNPROCESSABLE_ENTITY {
            let refusal: Refusal = response.json().await?;
            return Err(ApiError::Refused(refusal.error));
        }
        response.error_for_status()?;
        Ok(())
    }
}

#[derive(Clone)]
struct AppState {
    api: Api,
}

struct NoteList {
    notes: Vec<Note>,
}

impl Fragment for NoteList {
    type Signals = NotesPage;
    const ID: &'static str = "note-list";

    fn render(&self) -> El<NotesPage> {
        ul().css(styles::list())
            .children(self.notes.iter().map(|note| {
                li().id(format!("note-{}", note.id))
                    .css(styles::item())
                    .text(note.text.as_str())
            }))
    }
}

mod styles {
    use hypersig::style::Style;
    use hypersig::style::value::{
        BorderBottomStyle, Color, Cursor, Display, FontFamily, ListStyle, MarginInline, px, rem,
    };

    pub fn page() -> Style {
        Style::new()
            .font_family(FontFamily::SystemUi)
            .max_width(rem(32))
            .margin_block(rem(3))
            .margin_inline(MarginInline::Auto)
            .padding_inline(rem(1))
    }

    pub fn form() -> Style {
        Style::new().display(Display::Flex).gap(rem(0.5))
    }

    pub fn input() -> Style {
        Style::new()
            .flex_grow(1)
            .font_size(rem(1))
            .padding(rem(0.4))
    }

    pub fn button() -> Style {
        Style::new().font_size(rem(1)).cursor(Cursor::Pointer)
    }

    pub fn list() -> Style {
        Style::new().list_style(ListStyle::None).padding(px(0))
    }

    pub fn item() -> Style {
        Style::new()
            .padding_block(rem(0.3))
            .border_bottom_width(px(1))
            .border_bottom_style(BorderBottomStyle::Solid)
            .border_bottom_color(Color::hex(0x00ee_eeee))
    }

    pub fn problem() -> Style {
        Style::new().color(Color::hex(0x00b4_2318))
    }
}

async fn index(state: AppState, (): ()) -> Result<Page<NotesPage>, ApiError> {
    let list = NoteList {
        notes: state.api.notes().await?,
    };
    let draft = NotesPage::draft();
    let problem = NotesPage::problem();
    Ok(Page::new(
        "hypersig api",
        NotesPage {
            draft: String::new(),
            problem: String::new(),
        },
    )
    .child(
        div()
            .css(styles::page())
            .child(h1().text("Notes"))
            .child(
                form()
                    .css(styles::form())
                    .on_submit(ADD.call_with(draft).error(problem))
                    .child(
                        input()
                            .css(styles::input())
                            .attr(attr::PLACEHOLDER, "Write a note")
                            .bind(draft),
                    )
                    .child(button().css(styles::button()).text("Add")),
            )
            .child(
                p().id("problem")
                    .attr(attr::ROLE, "alert")
                    .css(styles::problem())
                    .show(!problem.is_empty())
                    .bind_text(problem),
            )
            .child(hypersig::fragment(&list)),
    ))
}

/// Sends the note to the service, then reads the list back from it, so the
/// page shows the notes the service holds.
async fn add(state: AppState, draft: String) -> Result<Patches<NotesPage>, ApiError> {
    state.api.add(&draft).await?;
    let list = NoteList {
        notes: state.api.notes().await?,
    };
    Ok(Patches::new()
        .element(&list)
        .signals(NotesPage::draft().patch(String::new()))
        .signals(NotesPage::problem().patch(String::new())))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let api = Api {
        client: reqwest::Client::new(),
        base: std::env::var("API_URL").unwrap_or_else(|_| "http://127.0.0.1:3013".to_owned()),
        token: std::env::var("API_TOKEN").unwrap_or_else(|_| "dev-token".to_owned()),
    };
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(ADD, add)
        .with_state(AppState { api });
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3012".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}
