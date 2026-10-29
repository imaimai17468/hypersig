use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Review {
    likes: i32,
    stars: i32,
    comment: String,
}

hypersig::routes! {
    fn pages(Router<()>);
    // `Action<Review>` takes no arguments, because `A` defaults to `()`.
    action LIKE: Action<Review> = post "/like" => like;
    // Two values travel as a tuple.
    action RATE: Action<Review, (i32, String)> = post "/rate" => rate;
}

async fn like((): (), (): ()) -> Patches<Review> {
    Review::likes().patch(1).into()
}

async fn rate((): (), (stars, comment): (i32, String)) -> Patches<Review> {
    Review::stars()
        .patch(stars)
        .and(Review::comment().patch(comment))
        .into()
}

let app: axum::Router = pages(axum::Router::new());
# let _ = app;
