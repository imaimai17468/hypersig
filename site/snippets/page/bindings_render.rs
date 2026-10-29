# use hypersig::html::{attr, button, p, span};
# use hypersig::{Signals, if_else};
# use serde::{Deserialize, Serialize};
use hypersig::Page;

# #[derive(Signals, Serialize, Deserialize)]
# struct Upload {
#     progress: i32,
#     busy: bool,
# }
# let progress = Upload::progress();
# let busy = Upload::busy();
let page = Page::new("Upload", Upload { progress: 0, busy: false })
    .child(span().bind_text(progress.to_text().concat(" %")))
    .child(
        button()
            .bind_attr(attr::ARIA_LABEL, if_else(busy, "Cancel the upload", "Nothing to cancel"))
            .attr_when(attr::DISABLED, !busy)
            .text("Cancel"),
    )
    .child(p().show(busy).text("Uploading…"));
let html = page.render();

// Each binding is evaluated with the initial values.
assert!(html.contains(">0 %</span>"));
assert!(html.contains(r#"aria-label="Nothing to cancel""#));
assert!(html.contains(r#"disabled="""#));
