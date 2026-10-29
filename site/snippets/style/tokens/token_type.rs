use hypersig::style::value::Color;
use hypersig::style::{Style, Token};

let brand = Token::new("brand", Color::hex(0x0025_63eb));
// error[E0277]: the trait bound `Width: From<Color>` is not satisfied
let bar = Style::new().width(brand.var());
