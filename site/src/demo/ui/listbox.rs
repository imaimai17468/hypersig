//! A listbox, for the listbox page.

use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::rem;
use hypersig::{Page, Segment, Signals};
use hypersig_ui::Listbox;
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub(super) enum Font {
    Sans,
    Serif,
    Mono,
}

#[derive(Signals, Serialize, Deserialize)]
pub(super) struct Editor {
    font: Font,
}

pub(super) fn page(site: Site) -> Page<Editor> {
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
                |font| match font {
                    Font::Sans => site.t(L {
                        en: "Sans serif",
                        ja: "サンセリフ",
                    }),
                    Font::Serif => site.t(L {
                        en: "Serif",
                        ja: "セリフ",
                    }),
                    Font::Mono => site.t(L {
                        en: "Monospace",
                        ja: "等幅",
                    }),
                },
            ),
        ),
    )
}
