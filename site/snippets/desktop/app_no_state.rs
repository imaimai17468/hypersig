# use axum::Router;
# use hypersig::{Page, Route, RouterExt, Signals};
# use hypersig_desktop::{App, Os};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Home {}
# #[derive(Clone)]
# struct AppState { os: Os }
# hypersig::routes! {
#     fn pages(Router<AppState>);
#     const INDEX: Route = "/" => index;
# }
# async fn index(state: AppState, (): ()) -> Page<Home> { let _ = state.os; Page::new("Home", Home {}) }
# const APP: App = App::new("dev.example.home");
# fn main() -> Result<(), hypersig_desktop::Error> {
// `pages` is a `Router<AppState>`, which still needs its state.
// error[E0277]: the trait bound `Router<AppState>: IntoRouter` is not satisfied
APP.run(&INDEX.url(()), |_os| async {
    pages(Router::new()).hypersig_runtime()
})
# }
