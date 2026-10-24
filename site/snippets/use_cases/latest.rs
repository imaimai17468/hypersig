# use std::time::Duration;
#
# use hypersig::html::{Event, div, input, p};
# use hypersig::{Action, Signals};
# use serde::{Deserialize, Serialize};
#
# #[derive(Signals, Serialize, Deserialize)]
# struct Search {
#     query: String,
#     results: Vec<String>,
#     searching: bool,
# }
#
# const FIND: Action<Search, String> = Action::get("/find");
#
# let query = Search::query();
let _ = div()
    .child(input().bind(query).on_debounced(
        Event::Input,
        Duration::from_millis(200),
//-        FIND.call_with(query),
        FIND.call_with(query).indicator(Search::searching()).latest(), //+
    ))
    .child(p().show(Search::searching()).text("Searching…")); //+
