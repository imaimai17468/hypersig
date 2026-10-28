use axum::http::StatusCode;
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Review {
    stars: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    action RATE: Action<Review, i32> = post "/rate" => rate;
}

// `Ok` answers 200 with the patches as an event stream, and `Err` answers
// its status with the text as a `text/plain` body.
async fn rate((): (), stars: i32) -> Result<Patches<Review>, (StatusCode, String)> {
    if !(1..=5).contains(&stars) {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, "Pick 1 to 5 stars.".to_owned()));
    }
    Ok(Review::stars().patch(stars).into())
}

# tokio::runtime::Runtime::new().unwrap().block_on(async {
let answer = rate((), 4).await.ok().map(|patches| patches.value_of(Review::stars()));
assert_eq!(answer, Some(Some(4)));
assert!(rate((), 9).await.is_err());
# });
# let _ = pages;
