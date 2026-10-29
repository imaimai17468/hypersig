use hypersig::style::Token;
use hypersig::style::value::Color;

// Panics: a token name is ASCII letters, digits, `-`, and `_`, not "brand color"
let brand = Token::new("brand color", Color::hex(0x0025_63eb));
# let _ = brand;
