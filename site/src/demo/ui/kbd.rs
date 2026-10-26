//! Keys and a shortcut in running text, for the kbd page.

use hypersig::html::{div, p, span};
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::kbd;
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Help {
    open: bool,
}

pub fn page(site: Site) -> Page<Help> {
    Page::new(
        site.t(L {
            en: "Help",
            ja: "ヘルプ",
        }),
        Help { open: false },
    )
    .child(
        div()
            .css(stack(rem(0.75)))
            .child(
                span()
                    .css(hstack(rem(0.25)))
                    .child(kbd().text("Ctrl"))
                    .child(kbd().text("Shift"))
                    .child(kbd().text("P")),
            )
            .child(
                p().text(site.t(L {
                    en: "Press ",
                    ja: "閉じるには ",
                }))
                .child(kbd().text("Esc"))
                .text(site.t(L {
                    en: " to close the dialog.",
                    ja: " を押してください。",
                })),
            ),
    )
}
