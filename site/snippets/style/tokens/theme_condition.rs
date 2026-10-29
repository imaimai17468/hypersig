use hypersig::style::value::Color;
use hypersig::style::{PseudoClass, Theme, Token};

let ink = Token::new("ink", Color::hex(0x0022_2222));
// error[E0308]: mismatched types: expected `Media`, found `PseudoClass`
let theme = Theme::new().on(PseudoClass::Hover, Theme::new().set(&ink, Color::BLACK));
