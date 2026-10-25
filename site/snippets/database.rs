# use axum::Router;
# use axum::http::StatusCode;
# use axum::response::{IntoResponse, Response};
# use hypersig::html::{li, ul};
# use hypersig::{Page, Route, RouterExt, Signals};
# use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use rusqlite::Connection;

#[derive(Signals, Serialize, Deserialize)]
struct Todos {
    count: i32,
}

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
}

#[derive(Debug)]
struct DbError(rusqlite::Error);

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        eprintln!("database: {}", self.0);
        let message = "The list could not be read. Try again.";
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}

async fn titles(state: &AppState) -> Result<Vec<String>, DbError> {
    let db = Arc::clone(&state.db);
    // rusqlite blocks the thread until SQLite answers.
    tokio::task::spawn_blocking(move || -> rusqlite::Result<Vec<String>> {
        let db = db.lock().expect("database lock poisoned");
        db.prepare("SELECT title FROM todos ORDER BY id")?
            .query_map([], |row| row.get(0))?
            .collect()
    })
    .await
    .expect("query panicked")
    .map_err(DbError)
}

async fn index(state: AppState, (): ()) -> Result<Page<Todos>, DbError> {
    let titles = titles(&state).await?;
    let count = i32::try_from(titles.len()).unwrap_or(i32::MAX);
    let list = ul().children(titles.into_iter().map(|title| li().text(title)));
    Ok(Page::new("Todos", Todos { count }).child(list))
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
}

let db = Connection::open("todos.db")?;
db.execute_batch(
    "CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY, title TEXT NOT NULL)",
)?;
let state = AppState { db: Arc::new(Mutex::new(db)) };
let _app: Router = pages(Router::new()).hypersig_runtime().with_state(state);
# Ok::<(), rusqlite::Error>(())
