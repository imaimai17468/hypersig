use hypersig::html::{div, h2, li, ul};
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Collapsible};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Releases {
    older: bool,
}

let older = Collapsible::new(Releases::older());

// The button sits in the heading, away from the list it shows.
let html = Page::new("Releases", Releases { older: false })
    .child(
        h2().text("Releases ").child(
            older
                .trigger(ButtonVariant::Ghost, ButtonSize::Sm)
                .text("Show older"),
        ),
    )
    .child(div().text("0.8.2 is the latest release."))
    .child(
        older
            .content()
            .child(ul().child(li().text("0.8.1")).child(li().text("0.8.0"))),
    )
    .render();
# assert!(html.contains(r#"aria-controls="ui-older-content" aria-expanded="false""#));
# assert!(html.contains(r#"<div id="ui-older-content""#));
