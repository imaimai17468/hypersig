use hypersig::style::Style;
use hypersig::style::value::pct;

// error[E0277]: the trait bound `BorderWidth: From<Percentage>` is not satisfied
let frame = Style::new().border_width(pct(1));
