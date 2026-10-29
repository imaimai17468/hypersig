use hypersig::html::{div, p};
use hypersig::style::Style;
use hypersig::style::value::TextAlign;
use hypersig::{Page, Signals};
use hypersig_ui::ToggleGroup;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Align {
    Start,
    Center,
    End,
}

#[derive(Signals, Serialize, Deserialize)]
struct Note {
    align: Align,
}

let align = Note::align();
let page = Page::new("Note", Note { align: Align::Start }).child(
    div()
        .child(
            ToggleGroup::new("Alignment", align)
                .item(Align::Start, "Start")
                .item(Align::Center, "Center")
                // A toggle that shows an icon is named by its label.
                .icon_item(Align::End, "End", "⇥"),
        )
        .child(
            p().css_match(
                align,
                [
                    (Align::Start, Style::new().text_align(TextAlign::Start)),
                    (Align::Center, Style::new().text_align(TextAlign::Center)),
                    (Align::End, Style::new().text_align(TextAlign::End)),
                ],
            )
            .text("The quick brown fox jumps over the lazy dog."),
        ),
);
# let html = page.render();
# assert!(html.contains(r#"role="radio" aria-checked="true" tabindex="0""#));
# assert!(html.contains(r#"aria-label="End""#));
