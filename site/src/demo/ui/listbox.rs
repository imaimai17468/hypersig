//! A listbox, for the listbox page.

use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::Listbox;
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Font {
    Sans,
    Serif,
    Mono,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Editor {
    font: Font,
}

pub fn page(site: Site) -> Page<Editor> {
    Page::new(
        site.t(L {
            en: "Editor",
            ja: "エディタ",
        }),
        Editor { font: Font::Sans },
    )
    .child(
        div().css(Style::new().max_width(rem(16))).child(
            Listbox::new(
                site.t(L {
                    en: "Font",
                    ja: "フォント",
                }),
                Editor::font(),
            )
            .option(
                Font::Sans,
                site.t(L {
                    en: "Sans serif",
                    ja: "サンセリフ",
                }),
            )
            .option(
                Font::Serif,
                site.t(L {
                    en: "Serif",
                    ja: "セリフ",
                }),
            )
            .option(
                Font::Mono,
                site.t(L {
                    en: "Monospace",
                    ja: "等幅",
                }),
            ),
        ),
    )
}
