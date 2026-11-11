use std::fs;

use axum::Router;
use hypersig::html::{button, div, p, pre};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
use hypersig_desktop::{App, Os};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Viewer {
    text: String,
    status: String,
}

#[derive(Clone)]
struct AppState {
    os: Os,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
    action OPEN: Action<Viewer> = post "/open" => open;
}

async fn index(_: AppState, (): ()) -> Page<Viewer> {
    let initial = Viewer { text: String::new(), status: String::new() };
    Page::new("Viewer", initial).child(
        div()
            .child(button().on_click(OPEN.call()).text("Open…"))
            .child(p().bind_text(Viewer::status()))
            .child(pre().bind_text(Viewer::text())),
    )
}

/// Asks for a text file and shows what it holds.
async fn open(state: AppState, (): ()) -> Patches<Viewer> {
    let chosen = state
        .os
        .dialog()
        .title("Open a note")
        .filter("Text", &["txt", "md"])
        .open_file()
        .await;
    let Some(path) = chosen else {
        // Cancelled: the page stays as it was.
        return Patches::new();
    };
    match fs::read_to_string(&path) {
        Ok(text) => Viewer::text()
            .patch(text)
            .and(Viewer::status().patch(path.display().to_string()))
            .into(),
        Err(error) => Viewer::status().patch(format!("Not read: {error}")).into(),
    }
}

const APP: App = App::new("dev.example.viewer");

fn main() -> Result<(), hypersig_desktop::Error> {
    APP.run(&INDEX.url(()), |os| async move {
        let state = AppState { os };
        pages(Router::new()).hypersig_runtime().with_state(state)
    })
}
