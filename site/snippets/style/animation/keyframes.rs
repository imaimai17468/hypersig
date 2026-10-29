use hypersig::html::{attr, span};
use hypersig::style::value::{AnimationIterationCount, AnimationTimingFunction, deg, ms};
use hypersig::style::{Keyframes, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Upload {
    busy: bool,
}

// @keyframes spin { 0% { rotate: 0deg } 100% { rotate: 360deg } }
let spin = Keyframes::new("spin")
    .at(0, Style::new().rotate(deg(0)))
    .at(100, Style::new().rotate(deg(360)));

let spinner = span::<Upload>()
    .attr(attr::ARIA_HIDDEN, "true")
    .css(
        Style::new()
            // animation-name: spin, and the @keyframes rule comes with it.
            .keyframes(&spin)
            .animation_duration(ms(800))
            .animation_timing_function(AnimationTimingFunction::Linear)
            .animation_iteration_count(AnimationIterationCount::Infinite),
    )
    .show(Upload::busy())
    .text("◌");

let html = Page::new("Upload", Upload { busy: true }).child(spinner).render();
assert!(html.contains("@keyframes spin{0%{rotate:0deg}100%{rotate:360deg}}"));
