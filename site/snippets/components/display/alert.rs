use hypersig::html::{div, when};
use hypersig::{Page, Signals};
use hypersig_ui::{AlertVariant, alert, alert_description, alert_title};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Builds {
    failed: bool,
    notice: String,
}

let signals = Builds { failed: true, notice: "They resume at 18:00 UTC.".into() };
let html = Page::new("Builds", signals)
    .child(
        div()
            // A status stays on the page and its text changes.
            .child(
                alert(AlertVariant::Default)
                    .child(alert_title().text("Deployments are paused"))
                    .child(alert_description().bind_text(Builds::notice())),
            )
            // An alert is read out as it is put on the page.
            .child(when(
                Builds::failed(),
                alert(AlertVariant::Destructive)
                    .child(alert_title().text("Build failed"))
                    .child(alert_description().text("The last commit does not compile.")),
            )),
    )
    .render();
# assert!(html.contains(r#"role="status""#));
# assert!(html.contains(r#"role="alert""#));
