use hypersig::html::button;
# use hypersig::{Action, Patches, Signals};
# use serde::{Deserialize, Serialize};
#
# #[derive(Signals, Serialize, Deserialize)]
# struct Review {
#     likes: i32,
#     stars: i32,
#     comment: String,
# }
#
# hypersig::routes! {
#     fn pages(Router<()>);
#     action LIKE: Action<Review> = post "/like" => like;
#     action RATE: Action<Review, (i32, String)> = post "/rate" => rate;
# }
# async fn like((): (), (): ()) -> Patches<Review> { Patches::new() }
# async fn rate((): (), _: (i32, String)) -> Patches<Review> { Patches::new() }

let like = button::<Review>().on_click(LIKE.call()).text("Like");

// The browser reads both signals when the button is clicked.
let args = (Review::stars(), Review::comment());
let rate = button::<Review>().on_click(RATE.call_with(args)).text("Send");
# let _ = (like, rate, pages);
