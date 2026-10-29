use hypersig::html::{CustomEvent, attr, el};
use hypersig::{Asset, Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Review {
    stars: i32,
}

const STAR_RATING: Asset = Asset::new("/star-rating.js");
// The script dispatches `rate` with the number of the star clicked.
const RATE: CustomEvent<i32> = CustomEvent::new("rate");

let stars = Review::stars();
let page = Page::new("Review", Review { stars: 3 })
    .script(STAR_RATING)
    .child(
        el(hypersig::tag!("star-rating"))
            .bind_attr(attr::VALUE, stars.to_text())
            .on_custom(RATE, move |rated| stars.set(rated)),
    );
# let html = page.render();
# assert!(html.contains(r#"<script type="module" src="/star-rating.js"></script>"#), "{html}");
# assert!(html.contains(r#"<star-rating value="3""#), "{html}");
