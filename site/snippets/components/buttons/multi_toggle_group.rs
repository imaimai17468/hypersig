use hypersig::html::{div, p};
use hypersig::style::Style;
use hypersig::style::value::{FontStyle, FontWeight, TextDecorationLine};
use hypersig::{Page, Signals};
use hypersig_ui::MultiToggleGroup;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Note {
    bold: bool,
    italic: bool,
    underline: bool,
}

let initial = Note { bold: true, italic: false, underline: false };
let page = Page::new("Note", initial).child(
    div()
        .child(
            MultiToggleGroup::new("Text style")
                .icon_item(Note::bold(), "Bold", "B")
                .icon_item(Note::italic(), "Italic", "I")
                .item(Note::underline(), "Underline"),
        )
        .child(
            p().css_when(Style::new().font_weight(FontWeight::Number(700.0)), Note::bold())
                .css_when(Style::new().font_style(FontStyle::Italic), Note::italic())
                .css_when(
                    Style::new().text_decoration_line(TextDecorationLine::Underline),
                    Note::underline(),
                )
                .text("The quick brown fox jumps over the lazy dog."),
        ),
);
# let html = page.render();
# assert!(html.contains(r#"role="group" aria-label="Text style""#));
# assert_eq!(html.matches(r#"tabindex="-1""#).count(), 2);
