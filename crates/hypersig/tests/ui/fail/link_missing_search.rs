//! A link to a route with search parameters that passes only the path's.

use hypersig::html::a;
use hypersig::{Page, Route, Search, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Search, Serialize, Deserialize)]
struct Filter {
    page: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    const TEAM: Route<String, Filter> = "/teams/{team}" => no_page;
}

async fn no_page<A>((): (), _: A) -> Option<Page<Filter>> {
    None
}

fn main() {
    let _ = pages;
    let _ = a::<Filter>().href(TEAM.link_with("red"));
}
