use hypersig::style::Style;
use hypersig::style::value::raw;

// error[E0277]: the trait bound `hypersig::hypersig_style::value::FontSize:
//     From<hypersig::hypersig_style::value::Raw>` is not satisfied
let title = Style::new().font_size(raw("clamp(1.5rem, 4vw, 3rem)"));
