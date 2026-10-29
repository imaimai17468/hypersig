use hypersig::style::Style;
use hypersig::style::value::CssWideKeyword;

// error[E0277]: the trait bound `hypersig::hypersig_style::value::Color: From<CssWideKeyword>`
//     is not satisfied
let link = Style::new().color(CssWideKeyword::Inherit);
