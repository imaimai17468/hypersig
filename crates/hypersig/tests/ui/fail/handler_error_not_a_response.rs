use axum::Router;
use hypersig::{Action, Patches, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

// An application error that does not say which response it answers with.
struct DatabaseError;

const ADD: Action<Counter> = Action::post("/add");

async fn add((): (), (): ()) -> Result<Patches<Counter>, DatabaseError> {
    Err(DatabaseError)
}

fn main() {
    let _: Router = Router::new().action(ADD, add);
}
