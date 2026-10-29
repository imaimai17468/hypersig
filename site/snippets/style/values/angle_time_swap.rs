use hypersig::style::Style;
use hypersig::style::value::ms;

// error[E0277]: the trait bound `hypersig::hypersig_style::value::Rotate:
//     From<hypersig::hypersig_style::value::Time>` is not satisfied
let tilt = Style::new().rotate(ms(90));
