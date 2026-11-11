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
use hypersig_desktop::{App, Os};

const APP: App = App::new("dev.example.notes");

/// Opens a window for each of `ids`, until one fails to open.
async fn show_all(os: &Os, ids: Vec<u64>) {
    for id in ids {
        if os.windows().open(&NOTE.url(id)).await.is_err() {
            break;
        }
    }
}

fn main() -> Result<(), hypersig_desktop::Error> {
    APP.run(&INDEX.url(()), |os| async move {
        show_all(&os, vec![1, 2]).await;
        pages(Router::new()).hypersig_runtime().with_state(AppState { os })
    })
}
