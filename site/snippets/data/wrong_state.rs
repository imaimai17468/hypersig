# use axum::Router;
# use hypersig::{Page, Route, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Todos {
#     count: i32,
# }
#[derive(Clone)]
struct AppState;

async fn index(_state: AppState, (): ()) -> Page<Todos> {
    Page::new("Todos", Todos { count: 0 })
}

hypersig::routes! {
    // The router's state is `()`, and `index` takes an `AppState`.
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
}
# let _: Router = pages(Router::new());
