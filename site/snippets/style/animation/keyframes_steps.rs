use hypersig::html::div;
use hypersig::style::value::{AnimationFillMode, Opacity, ms, pct};
use hypersig::style::{Keyframes, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {}

// The box grows past its size at 60% and settles at 100%. `opacity` has no
// 100% step, so it ends at the element's own value.
let pop_in = Keyframes::new("pop-in")
    .at(0, Style::new().opacity(Opacity::Number(0.0)).scale(pct(80)))
    .at(60, Style::new().opacity(Opacity::Number(1.0)).scale(pct(105)))
    .at(100, Style::new().scale(pct(100)));

let toast = div::<Inbox>()
    .css(
        Style::new()
            .keyframes(&pop_in)
            .animation_duration(ms(300))
            .animation_delay(ms(150))
            // Shows the 0% step during the delay instead of the final state.
            .animation_fill_mode(AnimationFillMode::Backwards),
    )
    .text("Message sent");
let html = Page::new("Inbox", Inbox {}).child(toast).render();
assert!(html.contains("60%{opacity:1;scale:105%}"));
