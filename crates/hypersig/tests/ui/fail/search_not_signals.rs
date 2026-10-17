//! Search parameters that are not a `#[derive(Signals)]` struct.

use hypersig::{Page, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Filter {
    page: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Blank {}

hypersig::routes! {
    fn pages(Router<()>);
    const FIND: Route<(), Filter> = "/find" => no_page;
}

async fn no_page<A>((): (), _: A) -> Option<Page<Blank>> {
    None
}

fn main() {
    let _ = pages;
    let _ = FIND.path();
}
