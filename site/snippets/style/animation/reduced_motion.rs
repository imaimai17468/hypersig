use hypersig::html::div;
use hypersig::style::value::{AnimationIterationCount, AnimationName, Opacity, Translate, ms, raw, rem};
use hypersig::style::{Keyframes, Media, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Feed {}

let pulse = Keyframes::new("pulse").at(50, Style::new().opacity(Opacity::Number(0.5)));
let slide = Keyframes::new("slide-in").at(0, Style::new().translate(Translate::Length(rem(1))));

// A loading placeholder: it stops pulsing for a visitor who asks for reduced
// motion, and stays at full opacity.
let skeleton = div::<Feed>().css(
    Style::new()
        .keyframes(&pulse)
        .animation_duration(ms(2000))
        .animation_iteration_count(AnimationIterationCount::Infinite)
        .on(
            Media::ReducedMotion,
            Style::new().animation_iteration_count(AnimationIterationCount::Number(0.0)),
        ),
);

// A card that slides in from the right, and appears in place under reduced motion.
let card = div::<Feed>().css(
    Style::new()
        .keyframes(&slide)
        .animation_duration(ms(300))
        .on(Media::ReducedMotion, Style::new().animation_name(AnimationName::None)),
);

// A color change that is instant under reduced motion.
let link = div::<Feed>().css(
    Style::new()
        .transition_property(raw("color"))
        .transition_duration(ms(150))
        .on(Media::ReducedMotion, Style::new().transition_duration(ms(0))),
);

let html = Page::new("Feed", Feed {})
    .child(skeleton)
    .child(card)
    .child(link)
    .render();
assert!(html.contains("{animation-name:none}"));
