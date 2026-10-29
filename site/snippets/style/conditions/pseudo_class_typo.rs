use hypersig::style::value::Color;
use hypersig::style::{PseudoClass, Style};

// error[E0599]: no variant, associated function, or constant named `Hovered` found for enum `PseudoClass`
let link = Style::new().on(PseudoClass::Hovered, Style::new().color(Color::RED));
