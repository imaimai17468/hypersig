use hypersig::style::Style;

// error[E0277]: the trait bound `hypersig::hypersig_style::value::Width: From<{integer}>`
//     is not satisfied
let sidebar = Style::new().width(240);
