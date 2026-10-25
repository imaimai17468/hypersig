use hypersig::html::{button, p, span, attr};
use hypersig::{Signals, if_else};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Upload {
    progress: i32,
    busy: bool,
}

let progress = Upload::progress();
let busy = Upload::busy();

// Text that follows a signal.
let label = span::<Upload>().bind_text(progress.to_text().concat(" %"));
// An attribute that follows a signal, and one that is present while a flag is true.
let cancel = button::<Upload>()
    .bind_attr(attr::ARIA_LABEL, if_else(busy, "Cancel the upload", "Nothing to cancel"))
    .attr_when(attr::DISABLED, !busy)
    .text("Cancel");
// An element shown only while the upload runs.
let notice = p::<Upload>().show(busy).text("Uploading…");
# let _ = (label, cancel, notice);
