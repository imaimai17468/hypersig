use hypersig::html::{div, span};
use hypersig::{Page, Signals};
use hypersig_ui::{BadgeVariant, badge};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Build {
    failed: bool,
}

let failed = Build::failed();
let html = Page::new("Build", Build { failed: false })
    .child(
        div()
            .child(span().text("main "))
            // The words carry the meaning, so the color is not the only cue.
            .child(badge(BadgeVariant::Destructive).show(failed).text("Failed"))
            .child(badge(BadgeVariant::Secondary).show(!failed).text("Passing"))
            .child(badge(BadgeVariant::Outline).text("v0.14")),
    )
    .render();
# assert!(html.contains(">Passing</span>"));
