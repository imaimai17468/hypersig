use axum::Router;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use hypersig::html::{button, p};
use hypersig::{Action, Patches, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    count: i32,
    problem: String,
}

struct NotesFull;

impl IntoResponse for NotesFull {
    fn into_response(self) -> Response {
        (StatusCode::CONFLICT, "You can keep 3 notes.").into_response()
    }
}

fn add_one(count: i32) -> Result<i32, NotesFull> {
    (count < 3).then_some(count + 1).ok_or(NotesFull)
}

const ADD: Action<Notes, i32> = Action::post("/notes");

async fn add((): (), count: i32) -> Result<Patches<Notes>, NotesFull> {
    let count = add_one(count)?;
    Ok(Notes::count().patch(count).into())
}

// The page shows the status and the body of a failed call.
let call = ADD.call_with(Notes::count()).error(Notes::problem());
let _add = button().on_click(call).text("Add");
let _problem = p().bind_text(Notes::problem());
let _app: Router = Router::new().action(ADD, add);
