# use axum::Router;
# use axum::http::StatusCode;
# use axum::response::{IntoResponse, Response};
# use hypersig::html::{li, ul};
# use hypersig::{Page, Route, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Notes {
#     count: i32,
# }
/// The client of the notes service. One `reqwest::Client` keeps a pool of
/// connections, so the router's state holds it and every handler shares it.
#[derive(Clone)]
struct AppState {
    client: reqwest::Client,
    base: String,
    token: String,
}

/// A note as the service's JSON has it.
#[derive(Deserialize)]
struct Note {
    text: String,
}

struct ApiError(reqwest::Error);

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        Self(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        eprintln!("notes service: {}", self.0);
        let message = "The notes service did not answer. Try again.";
        (StatusCode::BAD_GATEWAY, message).into_response()
    }
}

async fn index(state: AppState, (): ()) -> Result<Page<Notes>, ApiError> {
    let notes: Vec<Note> = state
        .client
        .get(format!("{}/notes", state.base))
        .bearer_auth(&state.token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let count = i32::try_from(notes.len()).unwrap_or(i32::MAX);
    let list = ul().children(notes.into_iter().map(|note| li().text(note.text)));
    Ok(Page::new("Notes", Notes { count }).child(list))
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
}

let state = AppState {
    client: reqwest::Client::new(),
    base: std::env::var("API_URL").expect("API_URL names the notes service"),
    token: std::env::var("API_TOKEN").expect("API_TOKEN holds its token"),
};
let _app: Router = pages(Router::new()).with_state(state);
