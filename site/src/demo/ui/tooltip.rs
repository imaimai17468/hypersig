//! A tooltip on an icon button, for the tooltip page.

use hypersig::html::div;
use hypersig::style::pattern::hstack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Tooltip};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Editor {
    bold: bool,
}

const BOLD: Tooltip<Editor> = Tooltip::new("bold-tip");
const SHARE: Tooltip<Editor> = Tooltip::new("share-tip");

pub fn page(site: Site) -> Page<Editor> {
    Page::new(
        site.t(L {
            en: "Editor",
            ja: "エディター",
        }),
        Editor { bold: false },
    )
    .child(
        div()
            .css(hstack(rem(0.75)))
            .child(
                BOLD.icon_trigger(
                    ButtonVariant::Outline,
                    site.t(L {
                        en: "Bold",
                        ja: "太字",
                    }),
                )
                .on_click(Editor::bold().toggle())
                .text("B"),
            )
            .child(BOLD.tooltip("Ctrl+B"))
            .child(
                SHARE
                    .trigger(ButtonVariant::Outline, ButtonSize::Default)
                    .text(site.t(L {
                        en: "Share",
                        ja: "共有",
                    })),
            )
            .child(SHARE.tooltip(site.t(L {
                en: "Copies a link to this document",
                ja: "このドキュメントへのリンクをコピーします",
            }))),
    )
}
