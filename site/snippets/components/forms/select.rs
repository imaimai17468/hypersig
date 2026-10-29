use hypersig::html::{div, p, span};
use hypersig::{Page, Signals};
use hypersig_ui::select_field;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Region {
    Tokyo,
    Frankfurt,
    Virginia,
}

#[derive(Signals, Serialize, Deserialize)]
struct Deploy {
    region: Region,
    replicas: i32,
}

let page = Page::new("Deploy", Deploy { region: Region::Tokyo, replicas: 2 }).child(
    div()
        .child(select_field(
            "Region",
            Deploy::region(),
            [
                (Region::Tokyo, "Tokyo"),
                (Region::Frankfurt, "Frankfurt"),
                (Region::Virginia, "Virginia"),
            ],
        ))
        // Any type that serializes works as a value, a number too.
        .child(select_field("Replicas", Deploy::replicas(), [(1, "One"), (2, "Two"), (3, "Three")]))
        .child(p().text("Replicas: ").child(span().bind_text(Deploy::replicas().to_text()))),
);
# let html = page.render();
# assert!(html.contains("<select"));
# assert!(html.contains("<option selected"));
