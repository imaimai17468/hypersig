use hypersig::html::attr;
use hypersig::html::span;
use hypersig::style::value::{AnimationIterationCount, AnimationTimingFunction, ms, raw};
use hypersig::style::{Keyframes, Media, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Upload {
    busy: bool,
}

let spin = Keyframes::new("spin")
    .at(0, Style::new().transform(raw("rotate(0deg)")))
    .at(100, Style::new().transform(raw("rotate(360deg)")));

let spinner = span::<Upload>()
    .attr(attr::ARIA_HIDDEN, "true")
    .css(
        Style::new()
            .keyframes(&spin)
            .animation_duration(ms(800))
            .animation_timing_function(AnimationTimingFunction::Linear)
            .animation_iteration_count(AnimationIterationCount::Infinite)
            // Slower for a visitor who asks for reduced motion.
            .on(Media::ReducedMotion, Style::new().animation_duration(ms(4000))),
    )
    .show(Upload::busy())
    .text("◌");

let html = Page::new("Upload", Upload { busy: true }).child(spinner).render();
assert!(html.contains("@keyframes spin"));
