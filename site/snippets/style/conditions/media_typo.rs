use hypersig::style::value::pct;
use hypersig::style::{Media, Style};

// error[E0599]: no variant, associated function, or constant named `Tablet` found for enum `Media`
let column = Style::new().on(Media::Tablet, Style::new().width(pct(50)));
