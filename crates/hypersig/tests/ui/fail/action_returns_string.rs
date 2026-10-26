//! An action's handler that returns a `String` in place of patches.

use hypersig::{Action, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Board {
    title: String,
}

hypersig::routes! {
    fn actions(Router<()>);
    action RENAME: Action<Board, String> = post "/rename" => rename;
}

async fn rename((): (), title: String) -> String {
    title
}

fn main() {
    let _ = actions;
}
