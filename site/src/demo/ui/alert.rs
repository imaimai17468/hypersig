//! A message alert and an error alert, for the alert page.

use hypersig::html::{div, when};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{
    AlertVariant, ButtonSize, ButtonVariant, alert, alert_description, alert_title, button,
};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Builds {
    failed: bool,
}

pub fn page() -> Page<Builds> {
    let failed = Builds::failed();
    Page::new("Builds", Builds { failed: false }).child(
        div()
            .css(stack(rem(1)))
            .child(
                alert(AlertVariant::Default)
                    .child(alert_title().text("Deployments are paused"))
                    .child(alert_description().text("They resume at 18:00 UTC.")),
            )
            .child(when(
                failed,
                alert(AlertVariant::Destructive)
                    .child(alert_title().text("Build failed"))
                    .child(alert_description().text("The last commit does not compile.")),
            ))
            .child(
                div().child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(failed.toggle())
                        .text("Toggle the error"),
                ),
            ),
    )
}
