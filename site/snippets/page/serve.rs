use axum::Router;
use hypersig::html::h1;
use hypersig::{Page, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    count: i32,
}

async fn notes((): (), (): ()) -> Page<Notes> {
    Page::new("Notes", Notes { count: 2 }).child(h1().text("Notes"))
}

hypersig::routes! {
    fn pages(Router<()>);
    const NOTES: Route = "/" => notes;
}

// The routes of the block, and the runtime at `hypersig::RUNTIME_PATH`.
let app: Router = pages(Router::new()).hypersig_runtime();
# let _ = (app, NOTES);
