use hypersig::style::Token;
use hypersig::style::value::FontFamily;

// error[E0277]: the trait bound `FontFamily: TokenValue` is not satisfied
let body = Token::new("body-font", FontFamily::SystemUi);
