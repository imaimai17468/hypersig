use hypersig::html::button;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Density {
    Comfortable,
    Compact,
}

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    density: Density,
    unread: i32,
}

let page = Page::new(
    "Inbox",
    Inbox {
        density: Density::Comfortable,
        unread: 3,
    },
)
// The next page load that persists `density` starts from the stored value.
.persist(Inbox::density())
.child(
    button()
        .on_click(Inbox::density().set(&Density::Compact))
        .text("Compact"),
);
assert!(page.render().contains("data-hs-persist"));
