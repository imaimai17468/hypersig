use std::time::Duration;

use hypersig::html::div;
use hypersig::{Expr, Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Toast, ToastVariant, Toaster, TextField, button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {
    toasts: Vec<Toast>,
    name: String,
}

let toasts = App::toasts();
let renamed = Toast::new("Renamed");
let html = Page::new("App", App { toasts: Vec::new(), name: "Report".into() })
    .child(
        div()
            .child(
                button(ButtonVariant::Outline, ButtonSize::Default)
                    .on_click(Toast::new("Saved").description("Your changes are live.").push(toasts))
                    .text("Save"),
            )
            .child(
                button(ButtonVariant::Outline, ButtonSize::Default)
                    .on_click(
                        Toast::new("Upload failed")
                            .description("The file is larger than 10 MB.")
                            .variant(ToastVariant::Destructive)
                            .push(toasts),
                    )
                    .text("Upload"),
            )
            .child(TextField::new("Name", App::name()))
            .child(
                button(ButtonVariant::Outline, ButtonSize::Default)
                    // The description is read from `name` when the button is pressed.
                    .on_click(Toast::push_expr(
                        toasts,
                        Expr::lit(&renamed).with(Toast::body(), App::name().concat(" is the new name.")),
                    ))
                    .text("Rename"),
            ),
    )
    .child(
        Toaster::new(toasts)
            .label("Notifications")
            .dismiss_label("Dismiss")
            .dismiss_after(Duration::from_secs(10)),
    )
    .render();
# assert!(html.contains(r#"aria-live="polite""#));
# assert!(html.contains(r#"aria-label="Dismiss""#));
