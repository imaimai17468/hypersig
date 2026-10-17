//! Styles under pseudo-classes, a marked ancestor, and a marked sibling,
//! for the page on conditions.

use hypersig::html::{a, div, input, label, p, span};
use hypersig::style::pattern::stack;
use hypersig::style::value::{
    BorderStyle, Color, FontWeight, Opacity, TextDecorationLine, px, rem,
};
use hypersig::style::{Condition, PseudoClass, Style};
use hypersig::{Page, Signals};
use hypersig_ui::token;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Settings {
    agreed: bool,
}

pub fn page() -> Page<Settings> {
    let card = a()
        .attr("href", "#")
        .css(
            stack(rem(0.25))
                .group()
                .padding(rem(1))
                .border_width(px(1))
                .border_style(BorderStyle::Solid)
                .border_color(token::border().var())
                .border_radius(px(8))
                .on(
                    PseudoClass::Hover,
                    Style::new().background_color(token::muted().var()),
                ),
        )
        .child(
            p().css(Style::new().font_weight(FontWeight::Number(600.0)))
                .text("Read the docs")
                .child(
                    span()
                        .css(Style::new().opacity(Opacity::Number(0.0)).on(
                            Condition::Group(PseudoClass::Hover),
                            Style::new().opacity(Opacity::Number(1.0)),
                        ))
                        .text(" →"),
                ),
        )
        .child(
            p().css(Style::new().color(token::muted_foreground().var()).on(
                Condition::Group(PseudoClass::Hover),
                Style::new().text_decoration_line(TextDecorationLine::Underline),
            ))
            .text("Hover anywhere on the card."),
        );
    let agree = label()
        .child(
            input()
                .attr("type", "checkbox")
                .css(Style::new().peer())
                .bind(Settings::agreed()),
        )
        .child(
            span()
                .css(Style::new().margin_left(rem(0.5)).on(
                    Condition::Peer(PseudoClass::Checked),
                    Style::new()
                        .color(Color::hex(0x0015_803d))
                        .font_weight(FontWeight::Number(600.0)),
                ))
                .text("I agree to the terms"),
        );
    Page::new("Settings", Settings { agreed: false }).child(
        div()
            .css(stack(rem(1.25)).max_width(rem(20)))
            .child(card)
            .child(agree),
    )
}
