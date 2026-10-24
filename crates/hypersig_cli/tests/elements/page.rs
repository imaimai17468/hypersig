//! A page that uses the module `cargo hypersig elements` writes from
//! `custom-elements.json`, which the test saves as `src/elements.rs`.

mod elements;

use elements::sl_rating::event::SlHoverDetail;
use elements::test_picker::event::PickDetail;
use elements::wa_stepper::event::WaStepChangeDetail;
use elements::{sl_button, sl_rating, test_picker, wa_stepper};
use hypersig::{Page, Signals, Stmt};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Rating {
    stars: f64,
    hovered: String,
    previous: Option<String>,
    kind: String,
    readonly: bool,
}

fn main() {
    let html = Page::new(
        "Rating",
        Rating {
            stars: 0.0,
            hovered: String::new(),
            previous: None,
            kind: String::new(),
            readonly: false,
        },
    )
    .child(
        sl_rating()
            .bind_attr(sl_rating::attr::VALUE, Rating::stars().to_text())
            .attr_when(sl_rating::attr::READONLY, Rating::readonly())
            .on_custom(sl_rating::event::SL_HOVER, |detail| {
                Rating::stars()
                    .set(detail.clone().at(SlHoverDetail::value()))
                    .then(Rating::hovered().set(detail.at(SlHoverDetail::phase())))
            })
            .on_custom(sl_rating::event::SL_CHANGE, |_| Stmt::none()),
    )
    .child(
        wa_stepper().on_custom(wa_stepper::event::WA_STEP_CHANGE, |detail| {
            Rating::previous().set(detail.at(WaStepChangeDetail::previousName()))
        }),
    )
    .child(test_picker().on_custom(test_picker::event::PICK, |detail| {
        Rating::kind().set(detail.at(PickDetail::r#type()))
    }))
    .child(sl_button().bind_attr(sl_button::attr::VARIANT, Rating::kind()))
    .render();
    println!("{html}");
}
