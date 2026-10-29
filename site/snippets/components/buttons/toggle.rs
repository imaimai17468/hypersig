use hypersig::html::{div, p};
use hypersig::style::Style;
use hypersig::style::value::{FontStyle, FontWeight};
use hypersig::{Page, Signals};
use hypersig_ui::{icon_toggle, toggle};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Note {
    bold: bool,
    italic: bool,
}

let page = Page::new("Note", Note { bold: true, italic: false }).child(
    div()
        // The text stays "Bold" in both states; `aria-pressed` says which.
        .child(toggle(Note::bold()).text("Bold"))
        .child(icon_toggle("Italic", Note::italic()).text("I"))
        .child(
            p().css_when(Style::new().font_weight(FontWeight::Number(700.0)), Note::bold())
                .css_when(Style::new().font_style(FontStyle::Italic), Note::italic())
                .text("The quick brown fox jumps over the lazy dog."),
        ),
);
# let html = page.render();
# assert!(html.contains(r#"aria-pressed="true""#));
# assert!(html.contains(r#"aria-label="Italic" aria-pressed="false""#));
