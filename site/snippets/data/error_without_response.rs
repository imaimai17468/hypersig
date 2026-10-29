# use axum::Router;
# use hypersig::{Page, Route, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Todos {
#     count: i32,
# }
# #[derive(Clone)]
# struct AppState;
#[derive(Debug)]
struct DbError(String);

// `DbError` has no `IntoResponse`, so the router has no answer for it.
async fn index(_state: AppState, (): ()) -> Result<Page<Todos>, DbError> {
    Err(DbError("no such table: todos".to_owned()))
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
}
# let _: Router<AppState> = pages(Router::new());
