use hypersig::html::{div, p};
use hypersig::style::Style;
use hypersig::style::value::FontFamily;
use hypersig::{Page, Segment, Signals};
use hypersig_ui::Listbox;
use serde::{Deserialize, Serialize};

// `Segment` lists the variants, in declaration order.
#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Font {
    Sans,
    Serif,
    Mono,
}

#[derive(Signals, Serialize, Deserialize)]
struct Editor {
    font: Font,
}

let font = Editor::font();
let page = Page::new("Editor", Editor { font: Font::Sans }).child(
    div()
        // A `match` names each variant, so a new variant without a title is an error.
        .child(Listbox::new("Font", font, |font| match font {
            Font::Sans => "Sans-serif",
            Font::Serif => "Serif",
            Font::Mono => "Monospace",
        }))
        .child(
            p().css_match(
                font,
                [
                    (Font::Sans, Style::new().font_family(FontFamily::SansSerif)),
                    (Font::Serif, Style::new().font_family(FontFamily::Serif)),
                    (Font::Mono, Style::new().font_family(FontFamily::Monospace)),
                ],
            )
            .text("The quick brown fox jumps over the lazy dog."),
        ),
);
# let html = page.render();
# assert!(html.contains(r#"role="listbox" aria-label="Font""#));
# assert!(html.contains(r#"role="option" aria-selected="true" tabindex="0""#));
