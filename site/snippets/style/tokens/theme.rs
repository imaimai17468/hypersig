use hypersig::html::p;
use hypersig::style::value::{Color, rem};
use hypersig::style::{Media, Style, Theme, Token};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Blog {}

let ink = Token::new("ink", Color::hex(0x0022_2222));
let paper = Token::new("paper", Color::WHITE);
let gap = Token::new("gap", rem(1));

// A brand's values, which a library's theme can be merged with.
let brand = Theme::new()
    .set(&ink, Color::hex(0x0011_1827))
    .set(&gap, rem(1.5))
    .on(
        Media::Dark,
        Theme::new().set(&ink, Color::hex(0x00f9_fafb)).set(&paper, Color::hex(0x0011_1827)),
    )
    .on(Media::Print, Theme::new().set(&ink, Color::BLACK));

let html = Page::new("Blog", Blog {})
    .theme(brand)
    // A later call adds to the theme, and replaces `--gap` under no condition.
    .theme(Theme::new().set(&gap, rem(2)))
    .child(
        p().css(Style::new().color(ink.var()).background_color(paper.var()).padding(gap.var()))
            .text("Hello"),
    )
    .render();
# assert!(html.contains(":root{--ink:#111827;--gap:2rem}"));
# assert!(html.contains("@media print{:root{--ink:black}}"));
# assert!(html.contains("@media (prefers-color-scheme: dark){:root{--ink:#f9fafb;--paper:#111827}}"));
