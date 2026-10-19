//! A star rating drawn by a custom element that a script defines, for the
//! page on what bindings and statements do not cover.

use hypersig::html::{CustomEvent, div, el, p};
use hypersig::style::pattern::hstack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals, if_else};
use hypersig_ui::{ButtonSize, ButtonVariant, button};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Review {
    stars: i32,
}

// `star-rating.js` dispatches it with the number of the star clicked.
const RATE: CustomEvent<i32> = CustomEvent::new("rate");

pub fn page(site: Site) -> Page<Review> {
    let stars = Review::stars();
    Page::new(
        site.t(L {
            en: "Rating",
            ja: "評価",
        }),
        Review { stars: 3 },
    )
    .script(site.star_rating())
    .child(
        div()
            .css(hstack(rem(1)))
            .child(
                el("star-rating")
                    .bind_attr("value", stars.to_text())
                    .on_custom(RATE, move |rated| stars.set(rated)),
            )
            .child(
                button(ButtonVariant::Outline, ButtonSize::Sm)
                    .on_click(stars.set(0))
                    .text(site.t(L {
                        en: "Clear",
                        ja: "クリア",
                    })),
            ),
    )
    .child(
        p().bind_text(if_else(
            stars.equals(0),
            site.t(L {
                en: "Not rated yet",
                ja: "まだ評価していません",
            }),
            stars.to_text().concat(site.t(L {
                en: " of 5 stars",
                ja: " / 5 つ星",
            })),
        )),
    )
}
