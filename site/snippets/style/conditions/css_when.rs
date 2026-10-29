use hypersig::html::li;
use hypersig::style::value::{Color, FontWeight, Opacity, TextDecorationLine, rem};
use hypersig::style::{PseudoClass, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Todo {
    done: bool,
}

let item = li::<Todo>()
    .css(Style::new().padding(rem(0.5)).color(Color::BLACK))
    // While `done` is true: overrides `color: black`.
    .css_when(
        Style::new()
            .color(Color::GRAY)
            .text_decoration_line(TextDecorationLine::LineThrough)
            .on(PseudoClass::Hover, Style::new().opacity(Opacity::Number(0.8))),
        Todo::done(),
    )
    // While `done` is false.
    .css_when(Style::new().font_weight(FontWeight::Number(600.0)), !Todo::done())
    .text("Buy milk");
# let html = Page::new("Todo", Todo { done: true }).child(item).render();
# assert!(html.contains("color_black when:color_gray when:text-decoration-line_line-through when:hover:opacity_0.8\""));
