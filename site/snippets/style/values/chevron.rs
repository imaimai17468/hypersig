use hypersig::html::{El, button, span};
use hypersig::style::value::{Display, TransitionTimingFunction, deg, ms, raw};
use hypersig::style::{Condition, PseudoClass, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Faq {}

/// A button whose arrow turns a quarter turn while the pointer is over it.
fn more() -> El<Faq> {
    button()
        .css(Style::new().group())
        .child(
            span()
                .css(
                    Style::new()
                        .display(Display::InlineBlock)
                        .transition_property(raw("rotate"))
                        .transition_duration(ms(200))
                        .transition_timing_function(TransitionTimingFunction::EaseOut)
                        .on(
                            Condition::Group(PseudoClass::Hover),
                            Style::new().rotate(deg(90)),
                        ),
                )
                .text("›"),
        )
        .text(" More")
}

let html = Page::new("Faq", Faq {}).child(more()).render();
# for css in ["{transition-duration:200ms}", "{rotate:90deg}", "{transition-timing-function:ease-out}"] {
#     assert!(html.contains(css), "{css}");
# }
