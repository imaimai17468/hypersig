//! A popover holding a form, for the popover page.

use hypersig::html::{div, p};
use hypersig::style::pattern::stack;
use hypersig::style::value::{FontWeight, rem};
use hypersig::style::Style;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Popover, TextField, button};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Layer {
    width: String,
}

hypersig::node_ref!(const SIZE_PANEL: NodeRef<Layer>);
const SIZE: Popover<Layer> = Popover::new(SIZE_PANEL);

pub fn page(site: Site) -> Page<Layer> {
    Page::new(
        site.t(L {
            en: "Layer",
            ja: "レイヤー",
        }),
        Layer {
            width: "100%".to_owned(),
        },
    )
    .child(
        div()
            .child(
                SIZE.trigger(ButtonVariant::Outline, ButtonSize::Default)
                    .text(site.t(L {
                        en: "Size",
                        ja: "サイズ",
                    })),
            )
            .child(
                SIZE.popover().child(
                    div()
                        .css(stack(rem(0.75)))
                        .child(
                            p().css(Style::new().font_weight(FontWeight::Number(500.0)))
                                .text(site.t(L {
                                    en: "Set the size of the layer.",
                                    ja: "レイヤーのサイズを設定します。",
                                })),
                        )
                        .child(TextField::new(
                            site.t(L {
                                en: "Width",
                                ja: "幅",
                            }),
                            Layer::width(),
                        ))
                        .child(
                            div().child(
                                button(ButtonVariant::Default, ButtonSize::Sm)
                                    .on_click(SIZE_PANEL.hide_popover())
                                    .text(site.t(L {
                                        en: "Done",
                                        ja: "完了",
                                    })),
                            ),
                        ),
                ),
            )
    )
}
