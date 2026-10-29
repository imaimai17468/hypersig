use hypersig::style::value::Opacity;
use hypersig::style::{Condition, Media, Style};

// error[E0308]: mismatched types: expected `PseudoClass`, found `Media`
let arrow = Style::new().on(Condition::Group(Media::Dark), Style::new().opacity(Opacity::Number(1.0)));
