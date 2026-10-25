# use hypersig::Signals;
# use serde::{Deserialize, Serialize};
use axum::Router;
use hypersig::{Page, Route, RouterExt};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Signals, Serialize, Deserialize)]
struct Review {
    stars: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
}

async fn index((): (), (): ()) -> Page<Review> {
    Page::new("Rating", Review { stars: 3 })
        .icon("/icon.svg")
        .script("/static/star-rating.js")
}

let app: Router = pages(Router::new())
    .hypersig_runtime()
    // One file at one path.
    .route_service("/icon.svg", ServeFile::new("assets/icon.svg"))
    // Each file under `static/` at the same path under `/static`.
    .nest_service("/static", ServeDir::new("static"));
# let _ = (app, INDEX);
