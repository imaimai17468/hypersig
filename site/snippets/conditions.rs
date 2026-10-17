use hypersig::style::value::{Color, Display, Opacity, pct, quoted, rem};
use hypersig::style::{Container, Media, PseudoClass, PseudoElement, Style};

let link = Style::new()
    .color(Color::hex(0x0025_63eb))
    // A pseudo-class.
    .on(PseudoClass::Hover, Style::new().color(Color::hex(0x001d_4ed8)))
    // Conditions nest: hovered and focused at once.
    .on(
        PseudoClass::Hover,
        Style::new().on(PseudoClass::FocusVisible, Style::new().opacity(Opacity::Number(0.8))),
    )
    // A pseudo-element.
    .pseudo(PseudoElement::After, Style::new().content(quoted(" →")));

let column = Style::new()
    .width(pct(100))
    // From 48rem wide, and from 64rem wide.
    .on(Media::Md, Style::new().width(pct(50)))
    .on(Media::Lg, Style::new().width(pct(33)))
    // The user's settings.
    .on(Media::ReducedMotion, Style::new().display(Display::Block))
    // The nearest query container, from 24rem wide.
    .on(Container::Sm, Style::new().padding(rem(2)));
# let _ = (link, column);
