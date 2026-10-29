use hypersig::html::{CustomEvent, el};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Review {
    comment: String,
}

const RATE: CustomEvent<i32> = CustomEvent::new("rate");

// The `detail` is an `i32`, and `comment` holds a `String`.
let rating = el::<Review>(hypersig::tag!("star-rating"))
    .on_custom(RATE, |rated| Review::comment().set(rated));
# let _ = rating;
