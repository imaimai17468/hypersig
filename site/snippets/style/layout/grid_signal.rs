use hypersig::html::{button, div};
use hypersig::style::pattern::grid;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Density {
    Comfortable,
    Compact,
}

#[derive(Signals, Serialize, Deserialize)]
struct Photos {
    density: Density,
}

// Each value of the signal picks one typed grid.
let gallery = div::<Photos>()
    .css(grid(3, rem(1)))
    .css_match(
        Photos::density(),
        [
            (Density::Comfortable, grid(3, rem(1))),
            (Density::Compact, grid(6, rem(0.25))),
        ],
    );
let toggle = button::<Photos>()
    .on_click(Photos::density().set(&Density::Compact))
    .text("Compact");
let html = Page::new("Photos", Photos { density: Density::Comfortable })
    .child(toggle)
    .child(gallery)
    .render();
assert!(html.contains("repeat(6, minmax(0, 1fr))"));
