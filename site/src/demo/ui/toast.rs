//! Toasts pushed from the browser, for the toast page.

use hypersig::html::div;
use hypersig::style::pattern::wrap;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Toast, ToastVariant, Toaster, button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct App {
    toasts: Vec<Toast>,
    next: i32,
}

pub fn page() -> Page<App> {
    let toasts = App::toasts();
    let next = App::next();
    let initial = App {
        toasts: Vec::new(),
        next: 0,
    };
    Page::new("App", initial).child(
        div()
            .css(wrap(rem(0.5)))
            .child(
                button(ButtonVariant::Outline, ButtonSize::Default)
                    .on_click(
                        Toast::new("Event created")
                            .description("Sunday, December 3 at 9:00")
                            .push(toasts, next)
                            .then(next.set(next + 1)),
                    )
                    .text("Show a toast"),
            )
            .child(
                button(ButtonVariant::Outline, ButtonSize::Default)
                    .on_click(
                        Toast::new("Upload failed")
                            .description("The file is larger than 10 MB.")
                            .variant(ToastVariant::Destructive)
                            .push(toasts, next)
                            .then(next.set(next + 1)),
                    )
                    .text("Show an error"),
            )
            .child(Toaster::new(toasts)),
    )
}
