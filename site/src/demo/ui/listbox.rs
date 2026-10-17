//! A listbox, for the listbox page.

use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::Listbox;
use serde::{Deserialize, Serialize};

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

pub fn page() -> Page<Editor> {
    Page::new("Editor", Editor { font: Font::Sans }).child(
        div().css(Style::new().max_width(rem(16))).child(
            Listbox::new("Font", Editor::font())
                .option(Font::Sans, "Sans serif")
                .option(Font::Serif, "Serif")
                .option(Font::Mono, "Monospace"),
        ),
    )
}
