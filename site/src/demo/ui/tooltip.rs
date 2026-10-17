//! A tooltip on an icon button, for the tooltip page.

use hypersig::html::div;
use hypersig::style::pattern::hstack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Tooltip};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Editor {
    bold: bool,
}

const BOLD: Tooltip<Editor> = Tooltip::new("bold-tip");
const SHARE: Tooltip<Editor> = Tooltip::new("share-tip");

pub fn page() -> Page<Editor> {
    Page::new("Editor", Editor { bold: false }).child(
        div()
            .css(hstack(rem(0.75)))
            .child(
                BOLD.icon_trigger(ButtonVariant::Outline, "Bold")
                    .on_click(Editor::bold().toggle())
                    .text("B"),
            )
            .child(BOLD.tooltip("Ctrl+B"))
            .child(
                SHARE
                    .trigger(ButtonVariant::Outline, ButtonSize::Default)
                    .text("Share"),
            )
            .child(SHARE.tooltip("Copies a link to this document")),
    )
}
