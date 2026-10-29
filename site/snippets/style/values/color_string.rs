use hypersig::style::Style;

// error[E0277]: the trait bound `hypersig::hypersig_style::value::Color: From<&str>`
//     is not satisfied
let link = Style::new().color("crimson");
