# use serde::{Deserialize, Serialize};
use hypersig::{Page, Route, Signals};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {}

# async fn home((): (), (): ()) -> Page<Shop> { Page::new("Shop", Shop {}) }
hypersig::routes! {
    // error[E0277]: `fn((), ()) -> impl Future<Output = Page<Shop>> {missing}`
    // cannot serve this page or action
    fn pages(Router<()>) else missing;
    const HOME: Route = "/" => home;
}

// The handler after `else` takes an `Unserved`, not `()`.
async fn missing((): (), (): ()) -> Page<Shop> {
    Page::new("Not found", Shop {})
}
# let _ = pages;
