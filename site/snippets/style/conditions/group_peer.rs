use hypersig::html::{a, div, input, label, span, attr};
use hypersig::style::value::{Color, Opacity};
use hypersig::style::{Condition, PseudoClass, Style};
use hypersig::{Page, SafeUrl, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Settings {
    agreed: bool,
}

// The arrow shows while the pointer is anywhere on the card.
let card = a::<Settings>()
    .href(SafeUrl::new("/docs"))
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
    .child(input().attr(attr::TYPE, "checkbox").css(Style::new().peer()).bind(Settings::agreed()))
    .child(
        span()
            .css(Style::new().on(Condition::Peer(PseudoClass::Checked), Style::new().color(Color::GREEN)))
            .text("I agree"),
    );
# let html = Page::new("Settings", Settings { agreed: false })
#     .child(div().child(card).child(agree))
#     .render();
# assert!(html.contains(":hover .group-hover\\:opacity_1{opacity:1}"));
# assert!(html.contains(":checked ~ .peer-checked\\:color_green{color:green}"));
