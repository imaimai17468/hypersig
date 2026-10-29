use std::time::Duration;

use hypersig::html::{Event, input};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
    searched: String,
}

let query = Search::query();

let field = input::<Search>()
    .bind(query)
    // Runs once the visitor has stopped typing for 300 ms.
    .on_debounced(Event::Input, Duration::from_millis(300), Search::searched().set(query));
# let html = Page::new("Search", Search { query: String::new(), searched: String::new() }).child(field).render();
# let _ = html;
