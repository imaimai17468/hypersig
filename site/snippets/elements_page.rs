# mod shoelace {
#     include!("../../snippets/elements_module.rs");
# }
# fn main() {
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};
use shoelace::sl_rating;
use shoelace::sl_rating::event::SlHoverDetail;

#[derive(Signals, Serialize, Deserialize)]
struct Review {
    stars: f64,
    hovered: f64,
}

let (stars, hovered) = (Review::stars(), Review::hovered());
let page = Page::new("Review", Review { stars: 3.0, hovered: 0.0 })
    .script("/shoelace/shoelace-autoloader.js")
    .child(
        sl_rating()
            .bind_attr(sl_rating::attr::VALUE, stars.to_text())
            .on_custom(sl_rating::event::SL_HOVER, move |hover| {
                hovered.set(hover.at(SlHoverDetail::value()))
            }),
    );
# let html = page.render();
# assert!(html.contains("sl-hover") && html.contains("<sl-rating"));
# }
