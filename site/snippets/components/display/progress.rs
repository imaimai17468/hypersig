use hypersig::html::div;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, progress};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Upload {
    percent: i32,
}

let percent = Upload::percent();
let html = Page::new("Upload", Upload { percent: 40 })
    .child(progress("Uploading report.pdf", percent))
    .child(
        div()
            .child(
                button(ButtonVariant::Secondary, ButtonSize::Sm)
                    .on_click(percent.set(percent - 10))
                    .text("-10%"),
            )
            .child(
                button(ButtonVariant::Secondary, ButtonSize::Sm)
                    .on_click(percent.set(percent + 10))
                    .text("+10%"),
            ),
    )
    .render();
# assert!(html.contains(r#"role="progressbar" aria-label="Uploading report.pdf" aria-valuemin="0" aria-valuemax="100" aria-valuenow="40""#));
# assert!(html.contains(r#"style="width: 40%""#));
