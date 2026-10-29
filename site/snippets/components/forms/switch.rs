use hypersig::html::{div, p};
use hypersig::{Page, Signals, if_else};
use hypersig_ui::switch;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Settings {
    airplane: bool,
    notifications: bool,
}

let notifications = Settings::notifications();
let page = Page::new("Settings", Settings { airplane: false, notifications: true }).child(
    div()
        .child(switch("Airplane mode", Settings::airplane()))
        .child(switch("Notifications", notifications))
        .child(p().bind_text(if_else(
            notifications,
            "You get a notice for each new message.",
            "Notices are paused.",
        ))),
);
# let html = page.render();
# assert!(html.contains(r#"type="checkbox" role="switch""#));
