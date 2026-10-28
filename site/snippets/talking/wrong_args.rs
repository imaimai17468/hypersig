use hypersig::html::button;
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Review {
    stars: i32,
    comment: String,
}

hypersig::routes! {
    fn pages(Router<()>);
    action RATE: Action<Review, (i32, String)> = post "/rate" => rate;
}
# async fn rate((): (), _: (i32, String)) -> Patches<Review> { Patches::new() }

// error[E0277]: `Sig<Review, i32>` is not an expression of type `(i32, String)` on the page whose signals are `Review`
let send = button::<Review>().on_click(RATE.call_with(Review::stars()));
# let _ = (send, pages);
