use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, TextField, button, card, card_content};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Signup {
    name: String,
}

let html = Page::new("Sign up", Signup { name: String::new() })
    // The reset, the font, and the page colors.
    .global(hypersig_ui::global())
    // The dark values of the tokens, for a system that prefers dark.
    .theme(hypersig_ui::theme())
    .child(
        card().child(
            card_content()
                .child(TextField::new("Name", Signup::name()))
                .child(button(ButtonVariant::Default, ButtonSize::Default).text("Continue")),
        ),
    )
    .render();
# assert!(html.contains(":root{color-scheme:light dark}"));
# assert!(html.contains("@media (prefers-color-scheme: dark){:root{--background:#09090b;"));
