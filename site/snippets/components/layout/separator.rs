use hypersig::html::{Orientation, div, p, span};
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::separator;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {}

let html = Page::new("Profile", Profile {})
    .child(
        div()
            .css(stack(rem(1)))
            .child(p().text("Typed hypermedia for Rust."))
            .child(separator(Orientation::Horizontal))
            // A row of fixed height, which the vertical lines stretch to.
            .child(
                div()
                    .css(hstack(rem(1)).height(rem(1.25)))
                    .child(span().text("Blog"))
                    .child(separator(Orientation::Vertical))
                    .child(span().text("Docs"))
                    .child(separator(Orientation::Vertical))
                    .child(span().text("Source")),
            ),
    )
    .render();
# assert!(html.contains("<hr"));
# assert_eq!(html.matches(r#"role="separator" aria-orientation="vertical""#).count(), 2);
