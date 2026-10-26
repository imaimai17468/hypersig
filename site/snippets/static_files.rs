# use hypersig::Signals;
# use serde::{Deserialize, Serialize};
use axum::Router;
use hypersig::{Asset, Page, Route, RouterExt};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Signals, Serialize, Deserialize)]
struct Review {
    stars: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
}

const ICON: Asset = Asset::new("/assets/icon.svg");
const STAR_RATING: Asset = Asset::new("/star-rating.js");

async fn index((): (), (): ()) -> Page<Review> {
    Page::new("Rating", Review { stars: 3 })
        .icon(ICON)
        .script(STAR_RATING)
}

let app: Router = pages(Router::new())
    .hypersig_runtime()
    // One file at one path.
    .route_service("/star-rating.js", ServeFile::new("star-rating.js"))
    // Each file under `assets/` at the same path under `/assets`.
    .nest_service("/assets", ServeDir::new("assets"));
# let _ = (app, INDEX);
