//! A route with more parameters than a tuple of `Segment`s can hold.

use hypersig::{Page, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Blank {}

hypersig::routes! {
    fn pages(Router<()>);
    const DEEP: Route<(i32, i32, i32, i32, i32, i32, i32)> = "/{a}/{b}/{c}/{d}/{e}/{f}/{g}" => no_page;
}

async fn no_page<A>((): (), _: A) -> Option<Page<Blank>> {
    None
}

fn main() {
    let _ = pages;
    let _ = DEEP.path();
}
