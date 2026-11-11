# use axum::Router;
# use hypersig::{Page, Route, RouterExt, Signals};
use hypersig_desktop::App;
# use serde::{Deserialize, Serialize};
use tower_sessions::cookie::SameSite;
use tower_sessions::{MemoryStore, SessionManagerLayer};
# #[derive(Signals, Serialize, Deserialize)]
# struct Home {}
# hypersig::routes! {
#     fn pages(Router<()>);
#     const HOME: Route = "/" => home;
# }
# async fn home((): (), (): ()) -> Page<Home> {
#     Page::new("Home", Home {})
# }

/// The application, whose session cookie is marked `Secure` when `secure` is true.
fn app(secure: bool) -> Router {
    let sessions = SessionManagerLayer::new(MemoryStore::default())
        .with_same_site(SameSite::Lax)
        .with_secure(secure);
    pages(Router::new()).hypersig_runtime().layer(sessions)
}

const APP: App = App::new("dev.example.members");

// The desktop build. The web build serves `app(true)`.
fn main() -> Result<(), hypersig_desktop::Error> {
    APP.run(&HOME.url(()), |_os| async { app(false) })
}
