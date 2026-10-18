// The handler after `else` receives an `Unserved`, not a route's parameters.
use hypersig::{Page, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {}

async fn order((): (), _id: u32) -> Option<Page<Shop>> {
    None
}

async fn missing((): (), _id: u32) -> Page<Shop> {
    Page::new("Not found", Shop {})
}

hypersig::routes! {
    fn pages(Router<()>) else missing;
    const ORDER: Route<u32> = "/orders/{id}" => order;
}

fn main() {
    let _ = pages;
}
