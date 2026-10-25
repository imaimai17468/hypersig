//! An action's handler that returns a `String` in place of patches.

use axum::Router;
use hypersig::{Action, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    title: String,
}

const RENAME: Action<Board, String> = Action::post("/rename");

async fn rename((): (), title: String) -> String {
    title
}

fn main() {
    let _: Router = Router::new().action(RENAME, rename);
}
