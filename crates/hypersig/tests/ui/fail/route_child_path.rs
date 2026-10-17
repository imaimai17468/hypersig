//! A child route whose path does not continue its parent's.

use hypersig::{Page, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Blank {}

hypersig::routes! {
    fn pages(Router<()>);
    const ORG: Route<String> = "/orgs/{org}" => no_page;
    const MEMBER: Route<(String, i32)> = "/teams/{team}/members/{id}" in ORG => no_page;
}

async fn no_page<A>((): (), _: A) -> Option<Page<Blank>> {
    None
}

fn main() {
    let _ = pages;
    let _ = MEMBER;
}
