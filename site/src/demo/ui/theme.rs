//! A theme picker over a card, for the theming page.

use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{
    ButtonSize, ButtonVariant, Level, button, card, card_content, card_description, card_header,
    card_title, select_field, switch, token,
};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Scheme {
    System,
    Light,
    Dark,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Preview {
    scheme: Scheme,
    on: bool,
}

pub fn page(site: Site) -> Page<Preview> {
    let initial = Preview {
        scheme: Scheme::System,
        on: true,
    };
    Page::new(
        site.t(L {
            en: "Preview",
            ja: "プレビュー",
        }),
        initial,
    )
    .child(
        div()
            .css(
                stack(rem(1))
                    .padding(rem(1))
                    .background_color(token::background().var())
                    .color(token::foreground().var()),
            )
            .css_match(
                Preview::scheme(),
                [
                    (Scheme::Light, hypersig_ui::light()),
                    (Scheme::Dark, hypersig_ui::dark()),
                ],
            )
            .child(
                div()
                    .css(Style::new().max_width(rem(12)))
                    .child(select_field(
                        site.t(L {
                            en: "Theme",
                            ja: "テーマ",
                        }),
                        Preview::scheme(),
                        [
                            (
                                Scheme::System,
                                site.t(L {
                                    en: "System",
                                    ja: "システム",
                                }),
                            ),
                            (
                                Scheme::Light,
                                site.t(L {
                                    en: "Light",
                                    ja: "ライト",
                                }),
                            ),
                            (
                                Scheme::Dark,
                                site.t(L {
                                    en: "Dark",
                                    ja: "ダーク",
                                }),
                            ),
                        ],
                    )),
            )
            .child(
                card()
                    .child(
                        card_header()
                            .child(card_title(Level::H2).text(site.t(L {
                                en: "Notifications",
                                ja: "通知",
                            })))
                            .child(card_description().text(site.t(L {
                                en: "Choose what reaches your inbox.",
                                ja: "受信トレイに届ける内容を選んでください。",
                            }))),
                    )
                    .child(
                        card_content()
                            .css(stack(rem(1)))
                            .child(switch(
                                site.t(L {
                                    en: "Product updates",
                                    ja: "製品のお知らせ",
                                }),
                                Preview::on(),
                            ))
                            .child(div().child(
                                button(ButtonVariant::Default, ButtonSize::Sm).text(site.t(L {
                                    en: "Save",
                                    ja: "保存",
                                })),
                            )),
                    ),
            ),
    )
}
