//! A link to a route with search parameters that passes only the path's.

use hypersig::html::a;
use hypersig::{Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Filter {
    page: i32,
}

const TEAM: Route<String, Filter> = Route::new("/teams/{team}");

fn main() {
    let _ = a::<Filter>().href(TEAM.link_with("red"));
}
