use hypersig::style::Style;
use hypersig::style::value::Color;

// error[E0277]: the trait bound `hypersig::hypersig_style::value::Width:
//     From<hypersig::hypersig_style::value::Color>` is not satisfied
let style = Style::new().width(Color::CRIMSON);
