//! An exclusive accordion, for the accordion page.

use hypersig::html::{div, p, span};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{Accordion, ButtonSize, ButtonVariant, button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Faq {
    helpful: i32,
}

pub fn page() -> Page<Faq> {
    let helpful = Faq::helpful();
    let answers = Accordion::exclusive("faq")
            .item(
                "Does it need JavaScript?",
                p().text("No. Each section is a <details>, which opens without it."),
            )
            .item(
                "Can I restyle it?",
                p().text("Set the tokens in a Theme, or add a Style to any element."),
            )
            .item(
                "Is it accessible?",
                p().text("The browser announces each summary as a button that opens."),
            );
    Page::new("FAQ", Faq { helpful: 0 }).child(
        div()
            .css(stack(rem(1)))
            .child(answers)
            .child(
                p().child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(helpful.set(helpful + 1))
                        .text("This helped"),
                )
                .text(" ")
                .child(span().bind_text(helpful.to_text()))
                .text(" found it helpful"),
            ),
    )
}
