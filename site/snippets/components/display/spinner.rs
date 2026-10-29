use hypersig::html::{div, p};
use hypersig::{Page, Signals};
use hypersig_ui::spinner;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Sync {
    busy: bool,
}

let busy = Sync::busy();
let html = Page::new("Sync", Sync { busy: true })
    .child(
        div()
            .child(spinner("Syncing").show(busy))
            .child(p().show(!busy).text("Up to date.")),
    )
    .render();
# assert!(html.contains(r#"aria-label="Syncing""#));
# assert!(html.contains(r#"role="status""#));
# assert!(html.contains(r#"aria-hidden="true""#));
