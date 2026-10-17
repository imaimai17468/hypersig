//! The button variants and sizes, for the button page.

use hypersig::html::{div, p, span};
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, icon_button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Buttons {
    clicks: i32,
}

pub fn page() -> Page<Buttons> {
    let clicks = Buttons::clicks();
    let count = clicks.set(clicks + 1);
    let variants = [
        (ButtonVariant::Default, "Default"),
        (ButtonVariant::Secondary, "Secondary"),
        (ButtonVariant::Outline, "Outline"),
        (ButtonVariant::Ghost, "Ghost"),
        (ButtonVariant::Destructive, "Destructive"),
        (ButtonVariant::Link, "Link"),
    ];
    Page::new("Buttons", Buttons { clicks: 0 }).child(
        div()
            .css(stack(rem(1)))
            .child(div().css(wrap(rem(0.5))).children(variants.into_iter().map(
                |(variant, text)| {
                    button(variant, ButtonSize::Default)
                        .on_click(count.clone())
                        .text(text)
                },
            )))
            .child(
                div()
                    .css(wrap(rem(0.5)))
                    .child(button(ButtonVariant::Outline, ButtonSize::Sm).text("Small"))
                    .child(button(ButtonVariant::Outline, ButtonSize::Lg).text("Large"))
                    .child(
                        button(ButtonVariant::Outline, ButtonSize::Default)
                            .attr("disabled", "")
                            .text("Disabled"),
                    )
                    .child(icon_button(ButtonVariant::Outline, "Add").text("+")),
            )
            .child(
                p().text("Clicked ")
                    .child(span().bind_text(clicks.to_text()))
                    .text(" times"),
            ),
    )
}
