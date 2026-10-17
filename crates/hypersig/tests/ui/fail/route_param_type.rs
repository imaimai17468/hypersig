use hypersig::{Page, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Blank {}

hypersig::routes! {
    fn pages(Router<()>);
    const PRICE: Route<f64> = "/prices/{amount}" => no_page;
}

async fn no_page<A>((): (), _: A) -> Option<Page<Blank>> {
    None
}

fn main() {
    let _ = pages;
}
