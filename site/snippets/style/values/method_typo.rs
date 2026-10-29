use hypersig::style::Style;
use hypersig::style::value::Color;

// error[E0599]: no method named `colour` found for struct
//     `hypersig::hypersig_style::Style` in the current scope
let style = Style::new().colour(Color::CRIMSON);
