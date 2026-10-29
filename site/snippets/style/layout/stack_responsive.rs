use hypersig::html::{button, div};
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::{JustifyContent, rem};
use hypersig::style::Media;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Checkout {}

// A column on a phone, a row from `md` up, with the buttons at its right end.
let actions = div::<Checkout>()
    .css(
        stack(rem(0.5))
            .on(Media::Md, hstack(rem(1)).justify_content(JustifyContent::FlexEnd)),
    )
    .child(button().text("Back"))
    .child(button().text("Pay now"));
let html = Page::new("Checkout", Checkout {}).child(actions).render();
assert!(html.contains("{flex-direction:row}"));
