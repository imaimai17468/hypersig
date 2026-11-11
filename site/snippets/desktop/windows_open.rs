# #[tokio::main]
# async fn main() {
use axum::Router;
use hypersig::{Action, Page, Patches, Route, Signals};
use hypersig_desktop::Os;
use hypersig_desktop::testing::FakeOs;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct ListPage {
    status: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct NotePage {
    id: u64,
}

#[derive(Clone)]
struct AppState {
    os: Os,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const NOTE: Route<u64> = "/notes/{id}" => note;
    action SHOW: Action<ListPage, u64> = post "/show" => show;
}

async fn note(_state: AppState, id: u64) -> Page<NotePage> {
    Page::new(format!("Note {id}"), NotePage { id })
}

/// Opens the note `id` in a window of its own.
async fn show(state: AppState, id: u64) -> Patches<ListPage> {
    match state.os.windows().open(&NOTE.url(id)).await {
        Ok(_window) => Patches::new(),
        Err(error) => ListPage::status()
            .patch(format!("The note could not be shown: {error}"))
            .into(),
    }
}

// `FakeOs` stands in for the system, so this opens no real window.
let fake = FakeOs::new();
let state = AppState { os: fake.os() };
show(state, 7).await;
assert_eq!(fake.open_windows(), [NOTE.url(7)]);
# let _ = pages(Router::new());
# }
