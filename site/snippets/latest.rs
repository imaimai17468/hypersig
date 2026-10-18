use std::time::Duration;

use hypersig::html::{Event, input};
use hypersig::{Action, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
    results: Vec<String>,
    searching: bool,
}

const FIND: Action<Search, String> = Action::get("/find");

let query = Search::query();
// A newer search aborts the one still waiting, so its answer is the one shown.
let field = input().bind(query).on_debounced(
    Event::Input,
    Duration::from_millis(200),
    FIND.call_with(query).indicator(Search::searching()).latest(),
);
# let _ = field;
