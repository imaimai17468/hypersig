use std::fs;

use axum::Router;
use hypersig::{Page, Route, RouterExt, Signals};
use hypersig_desktop::{App, Os};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Home {
    greeting: String,
}

#[derive(Clone)]
struct AppState {
    greeting: String,
    // The handlers open windows and dialogs through this.
    os: Os,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
}

async fn index(state: AppState, (): ()) -> Page<Home> {
# let _ = &state.os;
    Page::new("Home", Home { greeting: state.greeting })
}

const APP: App = App::new("dev.example.home")
    .size(800, 600)
    .min_size(480, 360)
    .resizable(true);

fn main() -> Result<(), hypersig_desktop::Error> {
    let file = APP.data_dir()?.join("greeting.txt");
    APP.run(&INDEX.url(()), |os| async move {
        // `run` made the directory before this future started, so the
        // read fails only when the file is missing.
        let greeting = fs::read_to_string(&file)?;
        let state = AppState { greeting, os };
        Ok::<_, std::io::Error>(pages(Router::new()).hypersig_runtime().with_state(state))
    })
}
