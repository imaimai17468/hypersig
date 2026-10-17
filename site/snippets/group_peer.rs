use hypersig::html::{a, div, input, label, span};
use hypersig::style::value::{Color, Opacity};
use hypersig::style::{Condition, PseudoClass, Style};
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Settings {
    agreed: bool,
}

// The arrow shows while the pointer is anywhere on the card.
let card = a::<Settings>()
    .attr("href", "/docs")
    .css(Style::new().group())
    .text("Read the docs")
    .child(
        span()
            .css(
                Style::new()
                    .opacity(Opacity::Number(0.0))
                    .on(Condition::Group(PseudoClass::Hover), Style::new().opacity(Opacity::Number(1.0))),
            )
            .text(" →"),
    );

// The text turns green while the checkbox before it is checked.
let agree = label::<Settings>()
    .child(input().attr("type", "checkbox").css(Style::new().peer()).bind(Settings::agreed()))
    .child(
        span()
            .css(Style::new().on(Condition::Peer(PseudoClass::Checked), Style::new().color(Color::GREEN)))
            .text("I agree"),
    );
# let _ = div::<Settings>().child(card).child(agree);
