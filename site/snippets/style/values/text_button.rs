use hypersig::html::{El, button, p};
use hypersig::style::value::{CssWideKeyword, Cursor, TextDecorationLine};
use hypersig::style::{Property, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Terms {}

/// A button that reads as a link inside a sentence: the browser's button
/// look removed, then the surrounding text's color and font.
fn text_button(label: &str) -> El<Terms> {
    button()
        .css(
            Style::new()
                .keyword(Property::All, CssWideKeyword::Unset)
                .keyword(Property::Color, CssWideKeyword::Inherit)
                .keyword(Property::Font, CssWideKeyword::Inherit)
                .text_decoration_line(TextDecorationLine::Underline)
                .cursor(Cursor::Pointer),
        )
        .text(label)
}

let html = Page::new("Terms", Terms {})
    .child(p().text("By signing up you accept the ").child(text_button("terms")))
    .render();
# for css in ["{all:unset}", "{color:inherit}", "{font:inherit}"] {
#     assert!(html.contains(css), "{css}");
# }
# assert!(html.find("{all:unset}") < html.find("{color:inherit}"));
