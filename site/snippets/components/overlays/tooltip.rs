use hypersig::html::div;
use hypersig::{NodeRef, Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Tooltip};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Editor {
    bold: bool,
}

hypersig::node_ref!(const BOLD_TIP: NodeRef<Editor>);
hypersig::node_ref!(const SHARE_TIP: NodeRef<Editor>);
const BOLD: Tooltip<Editor> = Tooltip::new(BOLD_TIP);
const SHARE: Tooltip<Editor> = Tooltip::new(SHARE_TIP);
# fn main() {

let html = Page::new("Editor", Editor { bold: false })
    .child(
        div()
            .child(
                BOLD.icon_trigger(ButtonVariant::Outline, "Bold")
                    .on_click(Editor::bold().toggle())
                    .text("B"),
            )
            .child(BOLD.tooltip("Ctrl+B"))
            .child(SHARE.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Share"))
            .child(SHARE.tooltip("Copies a link to this document")),
    )
    .render();
# assert!(html.contains(r#"aria-label="Bold""#));
# assert!(html.contains(r#"popover="hint" role="tooltip""#));
# }
