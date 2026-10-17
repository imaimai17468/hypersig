//! A text field and a greeting that follows it, for the page on describing
//! the page.

use hypersig::html::{div, p, span};
use hypersig::style::Style;
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::TextField;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Greeting {
    name: String,
}

pub fn page() -> Page<Greeting> {
    let name = Greeting::name();
    let typed = !name.trim().is_empty();
    Page::new(
        "Greeting",
        Greeting {
            name: String::new(),
        },
    )
    .child(
        div()
            .css(stack(rem(1)).max_width(rem(20)))
            .child(TextField::new("Your name", name))
            .child(p().show(!typed.clone()).text("Type your name above."))
            .child(
                p().show(typed)
                    .text("Hello, ")
                    .child(span().bind_text(name.trim()))
                    .text("!"),
            )
            .child(
                p().css(Style::new().font_size(rem(0.875)))
                    .show(name.char_count().gt(12))
                    .text("That is a long name."),
            ),
    )
}
