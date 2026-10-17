//! A `{*name}` segment whose value is not `Rest`.

use hypersig::{Page, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Blank {}

hypersig::routes! {
    fn pages(Router<()>);
    const FILE: Route<String> = "/files/{*path}" => no_page;
}

async fn no_page<A>((): (), _: A) -> Option<Page<Blank>> {
    None
}

fn main() {
    let _ = pages;
}
