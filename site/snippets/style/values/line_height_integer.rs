use hypersig::style::Style;

// error[E0277]: the trait bound `hypersig::hypersig_style::value::LineHeight: From<{integer}>`
//     is not satisfied
// help: `LineHeight` implements trait `From<T>`: `From<Length>`, `From<Percentage>`, `From<f64>`
let body = Style::new().line_height(16);
