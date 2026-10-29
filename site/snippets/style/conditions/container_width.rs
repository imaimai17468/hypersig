use hypersig::style::value::{FlexDirection, rem};
use hypersig::style::{Container, Style};

// error[E0599]: no variant, associated function, or constant named `MinWidth` found for enum `Container`
let card = Style::new().on(Container::MinWidth(rem(30)), Style::new().flex_direction(FlexDirection::Row));
