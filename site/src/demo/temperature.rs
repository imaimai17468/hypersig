//! A temperature converter, for the page on expressions and statements.

use hypersig::html::{div, input, label, p, span};
use hypersig::style::Style;
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::{BorderStyle, px, rem};
use hypersig::{Page, Signals, if_else};
use hypersig_ui::{ButtonSize, ButtonVariant, button, token};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Weather {
    celsius: f64,
}

pub fn page() -> Page<Weather> {
    let celsius = Weather::celsius();
    let fahrenheit = celsius * 1.8 + 32.0;
    Page::new("Weather", Weather { celsius: 21.0 }).child(
        div()
            .css(stack(rem(0.75)))
            .child(
                label().css(hstack(rem(0.5))).text("Celsius").child(
                    input().attr("type", "number").bind(celsius).css(
                        Style::new()
                            .width(rem(6))
                            .padding_inline(rem(0.5))
                            .border_width(px(1))
                            .border_style(BorderStyle::Solid)
                            .border_color(token::input().var())
                            .border_radius(px(6)),
                    ),
                ),
            )
            .child(
                p().text("That is ")
                    .child(span().bind_text(fahrenheit.to_fixed(1)))
                    .text(" °F, ")
                    .child(span().bind_text(if_else(
                        celsius.le(0.0),
                        "freezing.",
                        "above freezing.",
                    ))),
            )
            .child(
                div()
                    .css(hstack(rem(0.5)))
                    .child(
                        button(ButtonVariant::Outline, ButtonSize::Sm)
                            .on_click(celsius.set(celsius - 10.0))
                            .text("-10"),
                    )
                    .child(
                        button(ButtonVariant::Outline, ButtonSize::Sm)
                            .on_click(celsius.set(celsius + 10.0))
                            .text("+10"),
                    )
                    .child(
                        button(ButtonVariant::Ghost, ButtonSize::Sm)
                            .on_click(celsius.set(0.0))
                            .text("Reset"),
                    ),
            ),
    )
}
