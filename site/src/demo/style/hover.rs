//! Styles under pseudo-classes, a marked ancestor, and a marked sibling,
//! for the page on conditions.

use hypersig::html::{a, div, input, label, p, span};
use hypersig::style::pattern::stack;
use hypersig::style::value::{
    BorderStyle, Color, FontWeight, Opacity, TextDecorationLine, px, rem,
};
use hypersig::style::{Condition, PseudoClass, Style};
use hypersig::{Page, SafeUrl, Signals};
use hypersig_ui::token;
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Settings {
    agreed: bool,
}

pub fn page(site: Site) -> Page<Settings> {
    let card = a()
        .href(SafeUrl::new("#"))
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
                .text(site.t(L {
                    en: "Read the docs",
                    ja: "ドキュメントを読む",
                }))
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
            .text(site.t(L {
                en: "Hover anywhere on the card.",
                ja: "カードのどこかにポインタを乗せてください。",
            })),
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
                .css(
                    Style::new().margin_left(rem(0.5)).on(
                        Condition::Peer(PseudoClass::Checked),
                        Style::new()
                            .color(Color::hex(0x0015_803d))
                            .font_weight(FontWeight::Number(600.0)),
                    ),
                )
                .text(site.t(L {
                    en: "I agree to the terms",
                    ja: "利用規約に同意します",
                })),
        );
    Page::new(
        site.t(L {
            en: "Settings",
            ja: "設定",
        }),
        Settings { agreed: false },
    )
    .child(
        div()
            .css(stack(rem(1.25)).max_width(rem(20)))
            .child(card)
            .child(agree),
    )
}
