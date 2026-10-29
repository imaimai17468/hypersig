use hypersig::style::Style;
use hypersig::style::value::Display;

// error[E0599]: no variant, associated function, or constant named `Flx`
// found for enum `hypersig::hypersig_style::value::Display`
let row = Style::new().display(Display::Flx);
