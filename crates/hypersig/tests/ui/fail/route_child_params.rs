//! A child route whose parameters do not start with its parent's.

use hypersig::{Page, PathSegment, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Blank {}

hypersig::routes! {
    fn pages(Router<()>);
    const ORG: Route<PathSegment> = "/orgs/{org}" => no_page;
    const MEMBER: Route<(i32, i32)> = "/orgs/{org}/members/{id}" in ORG => no_page;
}

async fn no_page<A>((): (), _: A) -> Option<Page<Blank>> {
    None
}

fn main() {
    let _ = pages;
    let _ = MEMBER;
}
