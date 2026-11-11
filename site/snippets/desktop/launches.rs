# use axum::Router;
# use hypersig::{Page, Route, RouterExt, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct ListPage {}
# #[derive(Signals, Serialize, Deserialize)]
# struct NotePage { id: u64 }
# #[derive(Clone)]
# struct AppState { os: Os }
# hypersig::routes! {
#     fn pages(Router<AppState>);
#     const INDEX: Route = "/" => index;
#     const NOTE: Route<u64> = "/notes/{id}" => note;
# }
# async fn index(state: AppState, (): ()) -> Page<ListPage> { let _ = state.os; Page::new("Notes", ListPage {}) }
# async fn note(_state: AppState, id: u64) -> Page<NotePage> { Page::new("Note", NotePage { id }) }
# /// Keeps `path` as an open note, and returns its id.
# fn keep(_path: std::path::PathBuf) -> u64 { 1 }
use hypersig_desktop::{App, Launches, Os};

const APP: App = App::new("dev.example.notes");

/// Opens a window for each file a later start names.
async fn open_launched(os: Os, mut launches: Launches) {
    while let Some(launch) = launches.next().await {
        for arg in launch.args() {
            let id = keep(launch.current_dir().join(arg));
            if os.windows().open(&NOTE.url(id)).await.is_err() {
                return;
            }
        }
    }
}

fn main() -> Result<(), hypersig_desktop::Error> {
    APP.run(&INDEX.url(()), |os| {
        // Taken before the router exists, so no launch is missed.
        tokio::spawn(open_launched(os.clone(), os.launches()));
        async move { pages(Router::new()).hypersig_runtime().with_state(AppState { os }) }
    })
}
