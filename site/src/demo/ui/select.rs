//! A select, for the select page.

use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::select_field;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Region {
    Tokyo,
    Frankfurt,
    Virginia,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Deploy {
    region: Region,
}

pub fn page() -> Page<Deploy> {
    Page::new("Deploy", Deploy { region: Region::Tokyo }).child(
        div().css(Style::new().max_width(rem(16))).child(select_field(
            "Region",
            Deploy::region(),
            [
                (Region::Tokyo, "Tokyo"),
                (Region::Frankfurt, "Frankfurt"),
                (Region::Virginia, "Virginia"),
            ],
        )),
    )
}
