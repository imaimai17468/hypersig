//! A token set from a signal, for the page on tokens and themes.

use hypersig::html::div;
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::{Color, px, rem};
use hypersig::style::{Style, Token};
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, radio_group, token};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Palette {
    accent: String,
}

pub fn page(site: Site) -> Page<Palette> {
    let accent = Token::new("accent-demo", Color::hex(0x0025_63eb));
    let swatch = Style::new()
        .width(rem(3))
        .height(rem(3))
        .border_radius(px(8))
        .background_color(accent.var());
    Page::new(
        site.t(L {
            en: "Palette",
            ja: "パレット",
        }),
        Palette {
            accent: "#2563eb".into(),
        },
    )
    .child(
        div()
            .css(stack(rem(1)))
            .bind_token(&accent, Palette::accent())
            .child(radio_group(
                site.t(L {
                    en: "Accent",
                    ja: "アクセント",
                }),
                Palette::accent(),
                [
                    (
                        "#2563eb".to_owned(),
                        site.t(L {
                            en: "Blue",
                            ja: "青",
                        }),
                    ),
                    (
                        "#16a34a".to_owned(),
                        site.t(L {
                            en: "Green",
                            ja: "緑",
                        }),
                    ),
                    (
                        "#e11d48".to_owned(),
                        site.t(L {
                            en: "Rose",
                            ja: "ローズ",
                        }),
                    ),
                ],
            ))
            .child(
                div()
                    .css(wrap(rem(0.75)))
                    .child(div().css(swatch.clone()))
                    .child(div().css(swatch.border_radius(px(999))))
                    .child(
                        button(ButtonVariant::Default, ButtonSize::Default)
                            // The button reads `primary`, which takes the accent here.
                            .css(Style::new().token(&token::primary(), accent.var()))
                            .text(site.t(L {
                                en: "Accent button",
                                ja: "アクセントのボタン",
                            })),
                    ),
            ),
    )
}
