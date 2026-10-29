use hypersig::html::{button, div};
use hypersig::style::value::{
    Color, Opacity, TransitionTimingFunction, Translate, Visibility, ms, raw, rem,
};
use hypersig::style::{PseudoClass, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Menu {
    open: bool,
}

// The background fades over 150ms when the pointer enters or leaves.
let toggle = button::<Menu>()
    .css(
        Style::new()
            .background_color(Color::WHITE)
            .transition_property(raw("background-color"))
            .transition_duration(ms(150))
            .on(PseudoClass::Hover, Style::new().background_color(Color::GAINSBORO)),
    )
    .on_click(Menu::open().toggle())
    .text("Menu");

// Closed, the panel is transparent, 0.5rem higher, and hidden from clicks and
// focus. While `open` is true, the style of `css_when` overrides all three,
// and the browser animates the change.
let panel = div::<Menu>()
    .css(
        Style::new()
            .opacity(Opacity::Number(0.0))
            .translate(Translate::Length(rem(-0.5)))
            .visibility(Visibility::Hidden)
            .transition_property(raw("opacity, translate, visibility"))
            .transition_duration(ms(200))
            .transition_timing_function(TransitionTimingFunction::EaseOut),
    )
    .css_when(
        Style::new()
            .opacity(Opacity::Number(1.0))
            .translate(Translate::Length(rem(0)))
            .visibility(Visibility::Visible),
        Menu::open(),
    )
    .text("Settings");

let html = Page::new("Menu", Menu { open: false })
    .child(toggle)
    .child(panel)
    .render();
assert!(html.contains("{transition-property:opacity, translate, visibility}"));
