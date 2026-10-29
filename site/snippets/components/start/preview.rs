use hypersig::html::div;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, switch};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Settings {
    dark: bool,
}

let dark = Settings::dark();
let html = Page::new("Settings", Settings { dark: true })
    .global(hypersig_ui::global())
    .theme(hypersig_ui::theme())
    .child(switch("Dark preview", dark))
    .child(
        // The dark tokens and `color-scheme: dark` while `dark` is true.
        div()
            .css_when(hypersig_ui::dark(), dark)
            .child(button(ButtonVariant::Default, ButtonSize::Default).text("Save")),
    )
    .render();
# assert!(html.contains("color-scheme:dark"));
# assert!(html.contains("--background:#09090b"));
