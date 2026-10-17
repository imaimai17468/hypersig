//! The badge variants, for the badge page.

use hypersig::html::div;
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{BadgeVariant, ButtonSize, ButtonVariant, badge, button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Build {
    failed: bool,
}

pub fn page() -> Page<Build> {
    let failed = Build::failed();
    Page::new("Build", Build { failed: false }).child(
        div()
            .css(stack(rem(1)))
            .child(
                div()
                    .css(wrap(rem(0.5)))
                    .child(badge(BadgeVariant::Default).text("Default"))
                    .child(badge(BadgeVariant::Secondary).text("Secondary"))
                    .child(badge(BadgeVariant::Outline).text("Outline"))
                    .child(badge(BadgeVariant::Destructive).show(failed).text("Failed"))
                    .child(badge(BadgeVariant::Secondary).show(!failed).text("Passing")),
            )
            .child(
                div().child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(failed.toggle())
                        .text("Toggle the build"),
                ),
            ),
    )
}
