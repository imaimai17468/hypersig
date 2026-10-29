use hypersig::html::{button, div, h2, p, small, span, strong};
use hypersig::style::pattern::{divider, hstack, spacer, stack, vstack, wrap};
use hypersig::style::value::{AlignItems, Color, px, rem};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Plan {}

// A row: the spacer pushes the button to the right end.
let header = div::<Plan>()
    .css(hstack(rem(0.75)))
    .child(h2().text("Team plan"))
    .child(div().css(spacer()))
    .child(button().text("Choose"));

// A row whose texts of two sizes share one baseline instead of the center.
let price = div::<Plan>()
    .css(hstack(rem(0.25)).align_items(AlignItems::Baseline))
    .child(strong().text("$12"))
    .child(small().text("per seat a month"));

// Labels in rows that wrap onto the next line when the box is too narrow.
let features = div::<Plan>()
    .css(wrap(rem(0.5)))
    .children(["SSO", "Audit log", "Priority support"].map(|f| span().text(f)));

// A column centered across its width.
let footer = div::<Plan>()
    .css(vstack(rem(0.25)))
    .child(p().text("Billed yearly"))
    .child(p().text("Cancel at any time"));

let card = div::<Plan>()
    .css(stack(rem(1)))
    .child(header)
    .child(price)
    .child(features)
    .child(div().css(divider(px(1), Color::LIGHTGRAY)))
    .child(footer);
let html = Page::new("Pricing", Plan {}).child(card).render();
assert!(html.contains("{align-items:baseline}"));
