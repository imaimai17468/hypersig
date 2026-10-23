//! A stand-in for a notes service that another team runs: a JSON API behind
//! a bearer token, which the `example-api` server calls. It keeps the notes
//! in memory, since the example is about the calls rather than the storage.
//!
//! Run with `cargo run -p example-api --bin notes-backend`. It listens on
//! `ADDR`, `127.0.0.1:3013` by default, and accepts the token in
//! `API_TOKEN`, `dev-token` by default.

use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

const MAX_CHARS: usize = 80;

#[derive(Clone, Serialize)]
struct Note {
    id: u64,
    text: String,
}

#[derive(Deserialize)]
struct NewNote {
    text: String,
}

#[derive(Serialize)]
struct Refusal {
    error: String,
}

/// A request the service does not carry out.
enum Rejection {
    Unauthorized,
    Invalid(String),
}

impl IntoResponse for Rejection {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            Self::Invalid(error) => {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(Refusal { error })).into_response()
            }
        }
    }
}

#[derive(Clone)]
struct Backend {
    token: Arc<str>,
    notes: Arc<Mutex<Vec<Note>>>,
}

impl Backend {
    fn authorized(&self, headers: &HeaderMap) -> Result<(), Rejection> {
        let expected = format!("Bearer {}", self.token);
        let given = headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());
        if given == Some(expected.as_str()) {
            Ok(())
        } else {
            Err(Rejection::Unauthorized)
        }
    }
}

async fn list(
    State(backend): State<Backend>,
    headers: HeaderMap,
) -> Result<Json<Vec<Note>>, Rejection> {
    backend.authorized(&headers)?;
    Ok(Json(
        backend.notes.lock().expect("notes lock poisoned").clone(),
    ))
}

async fn create(
    State(backend): State<Backend>,
    headers: HeaderMap,
    Json(note): Json<NewNote>,
) -> Result<(StatusCode, Json<Note>), Rejection> {
    backend.authorized(&headers)?;
    let text = note.text.trim().to_owned();
    if text.is_empty() || text.chars().count() > MAX_CHARS {
        let error = format!("A note holds 1 to {MAX_CHARS} characters.");
        return Err(Rejection::Invalid(error));
    }
    let mut notes = backend.notes.lock().expect("notes lock poisoned");
    let id = notes.last().map_or(1, |last| last.id + 1);
    let note = Note { id, text };
    notes.push(note.clone());
    Ok((StatusCode::CREATED, Json(note)))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let token = std::env::var("API_TOKEN").unwrap_or_else(|_| "dev-token".to_owned());
    let backend = Backend {
        token: token.into(),
        notes: Arc::default(),
    };
    let app = Router::new()
        .route("/notes", get(list).post(create))
        .with_state(backend);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3013".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("notes backend listening on http://{addr}");
    axum::serve(listener, app).await
}
