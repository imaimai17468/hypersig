use hypersig::style::value::Color;
use hypersig::style::{PseudoElement, Style};

// Panics: a pseudo-element holds no pseudo-element
let style = Style::new().pseudo(
    PseudoElement::Before,
    Style::new().pseudo(PseudoElement::Marker, Style::new().color(Color::RED)),
);
# let _ = style;
