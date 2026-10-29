use hypersig::html::{div, input, attr};
use hypersig::style::value::{Color, rem};
use hypersig::style::{Style, Token};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Editor {
    accent: String,
}

let accent = Token::new("accent", Color::hex(0x0025_63eb));

let html = Page::new("Editor", Editor { accent: "#2563eb".into() })
    .child(
        div()
            // `style="--accent: #2563eb"`, rewritten on every change to `accent`.
            .bind_token(&accent, Editor::accent())
            .child(input().attr(attr::TYPE, "color").bind(Editor::accent()))
            .child(div().css(Style::new().height(rem(2)).background_color(accent.var()))),
    )
    .render();
# assert!(html.contains("--accent: #2563eb"));
