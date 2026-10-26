use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

// An application error that does not say which response it answers with.
struct DatabaseError;

hypersig::routes! {
    fn actions(Router<()>);
    action ADD: Action<Counter> = post "/add" => add;
}

async fn add((): (), (): ()) -> Result<Patches<Counter>, DatabaseError> {
    Err(DatabaseError)
}

fn main() {
    let _ = actions;
}
