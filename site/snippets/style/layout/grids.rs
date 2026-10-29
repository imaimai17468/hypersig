use hypersig::html::{article, div, h3, p};
use hypersig::style::pattern::{grid, grid_item, stack};
use hypersig::style::value::rem;
use hypersig::style::{Media, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Dashboard {}

let card = |title: &str| {
    article::<Dashboard>()
        .css(stack(rem(0.5)))
        .child(h3().text(title))
        .child(p().text("…"))
};

// One column on a phone, four from the `md` breakpoint (48rem) up.
let cards = div::<Dashboard>()
    .css(grid(1, rem(1)).on(Media::Md, grid(4, rem(1.5))))
    // From `md` up, the chart spans two of the four columns.
    .child(card("Revenue").css(Style::new().on(Media::Md, grid_item(2))))
    .child(card("Visitors"))
    .child(card("Signups"));
let html = Page::new("Dashboard", Dashboard {}).child(cards).render();
assert!(html.contains("{grid-template-columns:repeat(4, minmax(0, 1fr))}"));
