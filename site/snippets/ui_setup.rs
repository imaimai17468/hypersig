use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, TextField, button, card, card_content};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Signup {
    name: String,
}

let html = Page::new("Sign up", Signup { name: String::new() })
    .global(hypersig_ui::global())
    .theme(hypersig_ui::theme())
    .child(
        card().child(
            card_content()
                .child(TextField::new("Name", Signup::name()))
                .child(button(ButtonVariant::Default, ButtonSize::Default).text("Continue")),
        ),
    )
    .render();
assert!(html.contains(r#"for="ui-name-input""#));
